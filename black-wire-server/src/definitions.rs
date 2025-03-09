#[derive(serde::Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub os: String,
    pub os_version: String
}