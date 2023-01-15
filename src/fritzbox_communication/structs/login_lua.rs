/// The SessionInfo response struct from the fritzbox login process.
/// 
/// Example XML:
/// <?xml version="1.0" encoding="utf-8"?>
/// <SessionInfo>
/// <SID>8401b0ba4382129a</SID>
/// <Challenge>2$60000$0c9b6cae942935f6cecd3e66b50efa05$6000$88daadebbd2cfd912333f36851048818</Challenge>
/// <BlockTime>0</BlockTime>
/// <Rights>
///     <Name>Dial</Name>
///     <Access>2</Access>
///     <Name>App</Name>
///     <Access>2</Access>
///     <Name>HomeAuto</Name>
///     <Access>2</Access>
///     <Name>BoxAdmin</Name>
///     <Access>2</Access>
///     <Name>Phone</Name>
///     <Access>2</Access>
///     <Name>NAS</Name>
///     <Access>2</Access>
/// </Rights>
/// <Users>
///     <User last="1">fritz2264</User>
/// </Users>
/// </SessionInfo>
#[derive(Deserialize, Debug)]
pub struct SessionInfo {
    #[serde(rename="SID")]
    pub s_id: String,
    #[serde(rename="Challenge")]
    pub challenge: String,
    #[serde(rename="BlockTime")]
    block_time: String,
    #[serde(rename="Users")]
    pub users: Vec<User>
}

/// User struct of the SessionInfo struct.
#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename="User")]
    pub name: String,
}