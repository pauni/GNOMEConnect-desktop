use std::io::{
    Write,
    BufRead,
    BufReader,
    BufWriter
};
use hostname::get_hostname;
use std;
use serde::Serialize;
use serde::de::DeserializeOwned;
use server::packets;




#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload_type", content = "payload_data", rename_all = "lowercase")]
pub enum ResponseType {
    Device(packets::Device)
}
