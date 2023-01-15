pub mod structs;

use hyper::{Body, Client, Response};
use serde_xml_rs::from_str;
use self::structs::{
    general::FritzboxCommunication,
    login_lua::SessionInfo,
    data_lua::Root
};
use ring::{digest, pbkdf2};
use std::{
    num::NonZeroU32,
    string::String
};
use data_encoding::HEXLOWER;
use hex;
use chrono::Utc;
use std::env;

/// Implementation for the FritzboxCommunication struct.
impl FritzboxCommunication {
    
    /// Get new FritboxCommunication object.
    pub async fn new() -> Result<FritzboxCommunication, String>  {
        // Get a new client that can be reused for all requests.
        let client = Client::new();

        // Get fritbox password from environment variable, otherwise return an error.
        let password;
        match env::var("FRITZBOX_PASSWORD") {
            Ok(val) => password = val,
            Err(_e) => {
                return Err("No Fritzbox password provided!".to_string());
            },
        }

        // Get the session ID.
        let session_id_response = FritzboxCommunication::get_session_id(&client, password.to_string()).await;
        let session_id;
        match session_id_response {
            Ok(val) => {
                session_id = val;
            },
            Err(err) => {
                return Err(err);
            }
        }

        // Return the FritzboxCommunication object.
        return Ok(FritzboxCommunication{session_id: session_id.to_string(), session_id_timestamp: Utc::now().timestamp(), client});    
    }

    async fn get_session_id(client: &Client<hyper::client::HttpConnector>, password: String) -> Result<String, String>{
        // Get the challenge that needs to be solved to login.
        let sesssion_info_result = FritzboxCommunication::get_init_session_info(&client).await;
        let session_info: SessionInfo;
        match sesssion_info_result {
            Ok(val) => {
                session_info = val;
            },
            Err(error) => {
                log::error!("Error getting the first session info containing the challenge: {}", error);
                return Err("No login possible!".to_string());
            }
        }

        // Get the session id by solving the challenge.
        let session_id_result = FritzboxCommunication::retrieve_session_id(password.to_string(), session_info, &client).await;
        let session_id: String;
        match session_id_result {
            Ok(session_id_ok) => {
                session_id = session_id_ok;
            },
            Err(error) => {
                log::error!("Error getting the session id: {}", error);
                return Err("No login possible! Maybe wrong password or use of unsupported login method?".to_string());
            }
        }

        // Check for invalid session ID.
        if session_id == "0000000000000000" {
            return Err("Invalid session ID returned from the fritzbox API (maybe wrong password or used method).".to_string());
        }

        return Ok(session_id);
    }

    /// Checks if the session ID needds to be refreshed and does so if needed.
    pub async fn renew(&mut self) -> Result<(), String> {
        let current_timestamp = Utc::now().timestamp();
    
        // 20 minutes * 60 seconds = 1200 seconds, therefore if the time between the current timestamp and the last timestamp is same
        // or higher than 1200 the session ID needs to be refreshed.
        if (self.session_id_timestamp - current_timestamp) >= 1200 {
            // Get fritbox password from the environment variable.
            let password;
            match env::var("FRITZBOX_PASSWORD") {
                Ok(val) => password = val,
                Err(_e) => {
                    return Err("No Fritzbox password provided!".to_string());
                },
            }
            let session_id_response = FritzboxCommunication::get_session_id(&self.client, password).await;
            match session_id_response {
                Ok(val) => {
                    // Save the new session ID.
                    self.session_id = val;
                },
                Err(err) => {
                    let msg = format!("Failed to refresh the session ID: {}", err);
                    return Err(msg);
                }
            }
        }

        // Set the new timespamp.
        self.session_id_timestamp = current_timestamp;
        Ok(())
    }

    /// Get the SessionInfo containing the challenge for the fritzbox login by performing a http get request.
    /// # Arguments
    ///
    /// * `client` - A hyper HttpConnector client for http requests.
    async fn get_init_session_info(client: &Client<hyper::client::HttpConnector>) -> Result<SessionInfo, String> {
        let uri = "http://fritz.box/login_sid.lua?version=2".parse().unwrap();
        let resp = client.get(uri).await;
        let resp_body;
        match resp {
            Ok(val) => {
                resp_body = val;
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Error performing http get request to get the session info containing the challenge!".to_string());
            }
        }

        // Get the SessionInfo object from the response body.
        let session_info_response = FritzboxCommunication::deserialize_session_info(resp_body).await;
        match session_info_response {
            Ok(session_info) => {
                return Ok(session_info);
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Error deserializing the response into the SessionInfo object (maybe changes in the api)!".to_string());
            }
        }
    }

    /// Get the session id for the fritzbox api by solving the challenge.
    /// # Arguments
    ///
    /// * `password` - A string for the fritzbox password.
    /// * `challenge` - A string for the challenge that needs to be solved.
    /// * `client` - A hyper HttpConnector client for http requests.
    async fn retrieve_session_id(password: String, session_info: SessionInfo, client: &Client<hyper::client::HttpConnector>) -> Result<String, String> {

        // Split the challenge into the necessary parts according to: https://avm.de/fileadmin/user_upload/Global/Service/Schnittstellen/AVM_Technical_Note_-_Session_ID_deutsch_2021-05-03.pdf
        let split = session_info.challenge.split("$");
        let vec: Vec<&str> = split.collect();
        if vec.len() != 5 {
            return Err(String::from("Unsupported challenge format!"));
        }

        // Perform the PBKDF2_HMAC_SHA256 hash in the way as it's described here: https://avm.de/fileadmin/user_upload/Global/Service/Schnittstellen/AVM_Technical_Note_-_Session_ID_deutsch_2021-05-03.pdf
        const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
        let mut pbkdf2_hash_1 = [0u8; CREDENTIAL_LEN];
        let iter1 = NonZeroU32::new(vec[1].to_string().parse::<u32>().unwrap()).unwrap();
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iter1,
            &hex::decode(vec[2]).unwrap(),
            password.as_bytes(),
            &mut pbkdf2_hash_1,
        );
        let mut pbkdf2_hash_2 = [0u8; CREDENTIAL_LEN];
        let iter2 = NonZeroU32::new(vec[3].to_string().parse::<u32>().unwrap()).unwrap();
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iter2,
            &hex::decode(vec[4]).unwrap(),
            &pbkdf2_hash_1,
            &mut pbkdf2_hash_2,
        );

        // Build the response as described here: https://avm.de/fileadmin/user_upload/Global/Service/Schnittstellen/AVM_Technical_Note_-_Session_ID_deutsch_2021-05-03.pdf
        let response = vec[4].to_string() + "$" + &HEXLOWER.encode(&pbkdf2_hash_2);
        let uri_string = format!("http://fritz.box/login_sid.lua?version=2&response={}&username={}", response, session_info.users[0].name);
        let uri_result = uri_string.parse();
        let uri;
        match uri_result {
            Ok(val) => {
                uri = val
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Error when trying to parse the uri containing the response with the hashed password (probably malformed uri for some reason)!".to_string());
            }
        }

        // Perform the http request with the response containing the solved challenge and hashed password.
        let resp = client.get(uri).await;
        let resp_body;
        match resp {
            Ok(val) => {
                resp_body = val;
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Error performing http get request to get the session id containing the hashed password!".to_string());
            }
        }
        
        // Get the SessionInfo object from the response body.
        let session_info_response = FritzboxCommunication::deserialize_session_info(resp_body).await;
        match session_info_response {
            Ok(session_info) => {
                return Ok(session_info.s_id);
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Error deserializing the response into the SessionInfo object (maybe changes in the api)!".to_string());
            }
        }
    }

    /// Get the SessionInfo object from a login response body.
    /// # Arguments
    ///
    /// * `response_body` - A response body object containing a xml.
    async fn deserialize_session_info(res: Response<Body>) -> Result<SessionInfo, String> {
        // Get the body as a string 
        let body = FritzboxCommunication::body_to_string(res).await;
        log::debug!("Response body: {}", body);
        // Deserialize the xml-string to the SessionInfo object. 
        let session_info_result:Result<SessionInfo, serde_xml_rs::Error> = from_str(&body);
        match session_info_result {
            Ok(val) => {
                return Ok(val);
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Unknown object return from the fritzbox login!".to_string());
            }
        }
    }

    /// Get the response body as a string.
    /// # Arguments
    ///
    /// * `req` - A response body object.
    async fn body_to_string(res: Response<Body>) -> String {
        let body_bytes = hyper::body::to_bytes(res.into_body()).await;
        String::from_utf8(body_bytes.unwrap().to_vec()).unwrap()
    }

    /// Get all kinds of general informations about the Fritzbox.
    pub async fn get_data(&mut self) -> Result<Root, String> {
        // Check if the session ID is still valid and if not get a new session ID.
        let result = self.renew().await;
        match result {
            Ok(_) => {},
            Err(err) => {
                return Err(err);
            }
        }

        // Build the uri with the session id for authorization.
        let uri_string = format!("http://fritz.box/data.lua?sid={}", self.session_id);
        let uri: hyper::Uri = uri_string.parse().unwrap();

        // Perform the http request.
        let resp = self.client.get(uri).await;
        let resp_body;
        match resp {
            Ok(val) => {
                resp_body = val;
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Failed to load the data. :( See the log for more information.".to_string());
            }
        }

        // Deserialize the response
        let resp_body_string = FritzboxCommunication::body_to_string(resp_body).await;
        let root = serde_json::from_str::<Root>(&resp_body_string);
        match root {
            Ok(val) => {
                return Ok(val);
            },
            Err(error) => {
                log::error!("Error: {}", error);
                return Err("Received unknown data. :( See the log for more information.".to_string());
            }
        }
    }
}