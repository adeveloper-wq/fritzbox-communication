use hyper::Client;

/// The struct holding all the necessary information to communicate with the Fritbox API.
pub struct FritzboxCommunication {
    /// Needed for authorization.
    pub session_id: String,

    /// The Session ID is valid for 20 minutes after getting it or after the last action,
    /// therefore to renew it in time the last time it got refreshed or used should be saved as a
    /// timestamp in seconds since January 1, 1970 0:00:00 UTC (UNIX timestamp).
    pub session_id_timestamp: i64,

    /// One client that can be reused for all requests.
    pub client: Client<hyper::client::HttpConnector>,
}
