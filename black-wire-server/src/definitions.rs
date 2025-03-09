use std::collections::HashMap;


#[derive(serde::Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub os: String,
    pub os_version: String
}
#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct KeyLog {
    pub id: String,
    pub keys: HashMap<String, Vec<String>>
}