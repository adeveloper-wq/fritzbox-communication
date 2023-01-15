/// Autamtically generated with https://transform.tools/json-to-rust-serde and the json response from http://fritz.box/data.lua?sid=...

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub pid: String,
    pub hide: Hide,
    pub time: Vec<Value>,
    pub data: Data,
    pub sid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hide {
    pub share_usb: bool,
    pub live_tv: bool,
    pub fax_set: bool,
    pub prov_serv: bool,
    pub dect_moni_ex: bool,
    pub rss: bool,
    pub mobile: bool,
    pub dect_rdio: bool,
    pub dect_mail: bool,
    pub dect_moni: bool,
    pub sso_set: bool,
    pub live_img: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub naslink: String,
    pub fritzos: Fritzos,
    pub webdav: String,
    #[serde(rename = "MANUAL_URL")]
    pub manual_url: String,
    pub language: String,
    #[serde(rename = "AVM_URL")]
    pub avm_url: String,
    pub usbconnect: String,
    pub foncalls: Foncalls,
    pub vpn: Vpn,
    pub internet: Internet,
    pub dsl: Dsl,
    #[serde(rename = "SERVICEPORTAL_URL")]
    pub serviceportal_url: String,
    pub comfort: Comfort,
    pub changelog: Changelog,
    #[serde(rename = "wlan_guest")]
    pub wlan_guest: WlanGuest,
    pub lan: Lan,
    pub usb: Usb,
    pub net: Net,
    pub fonnum: Fonnum,
    #[serde(rename = "NEWSLETTER_URL")]
    pub newsletter_url: String,
    pub tamcalls: Tamcalls,
    pub dect: Dect,
    pub wlan: Wlan,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fritzos {
    #[serde(rename = "Productname")]
    pub productname: String,
    #[serde(rename = "NoPwd")]
    pub no_pwd: bool,
    #[serde(rename = "ShowDefaults")]
    pub show_defaults: bool,
    #[serde(rename = "expert_mode")]
    pub expert_mode: String,
    #[serde(rename = "fb_name")]
    pub fb_name: String,
    pub nspver: String,
    pub is_labor: bool,
    #[serde(rename = "twofactor_disabled")]
    pub twofactor_disabled: bool,
    #[serde(rename = "FirmwareSigned")]
    pub firmware_signed: bool,
    pub show_update: bool,
    pub is_update_avail: bool,
    pub energy: String,
    pub box_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Foncalls {
    pub calls: Vec<Call>,
    pub calls_today: String,
    #[serde(rename = "count_all")]
    pub count_all: i64,
    pub activecalls: String,
    #[serde(rename = "count_today")]
    pub count_today: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub number: String,
    pub link: String,
    pub date: String,
    pub duration: String,
    pub addible: bool,
    pub classes: String,
    pub name: String,
    pub display: String,
    pub fonname: String,
    pub unknown: bool,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vpn {
    pub elements: Vec<Value>,
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Internet {
    pub txt: Vec<String>,
    pub led: String,
    pub title: String,
    pub up: String,
    pub link2: String,
    pub down: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dsl {
    pub txt: String,
    #[serde(rename = "diag_stop_pid")]
    pub diag_stop_pid: String,
    pub link: String,
    pub led: String,
    pub title: String,
    pub add_diag: String,
    pub up: String,
    pub down: String,
    #[serde(rename = "diag_active")]
    pub diag_active: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comfort {
    pub func: Vec<Func>,
    pub any_comfort: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Func {
    pub linktxt: String,
    pub details: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changelog {
    pub device_name: String,
    pub fritz_os_version: String,
    pub connection_status: bool,
    pub product_name: String,
    pub iframe_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WlanGuest {
    pub txt: String,
    pub led: String,
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lan {
    pub txt: String,
    pub led: String,
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usb {
    pub txt: String,
    pub led: String,
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub any_unmeshed_devices: bool,
    pub count: i64,
    #[serde(rename = "more_link")]
    pub more_link: String,
    #[serde(rename = "active_count")]
    pub active_count: i64,
    pub devices: Vec<Device>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub classes: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub url: String,
    pub realtimeprio: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fonnum {
    pub txt: String,
    pub led: String,
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tamcalls {
    pub calls: String,
    #[serde(rename = "tam_configured")]
    pub tam_configured: bool,
    pub count: i64,
    pub calls_today: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dect {
    pub txt: String,
    pub led: String,
    pub title: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wlan {
    pub txt: String,
    pub led: String,
    pub title: String,
    pub link: String,
    pub tooltip: String,
}
