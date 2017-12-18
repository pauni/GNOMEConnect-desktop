pub mod request;
pub mod response;


use std::io::{
    Write,
    BufRead,
    BufReader,
    BufWriter
};
use hostname::get_hostname;
use std;
use serde::Serialize;
use serde::Deserialize;
use serde::de::DeserializeOwned;








#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportPacket {
    pub fingerprint: String,
    pub version: String,
    #[serde(rename = "payload")]
    pub payload: Payload,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload_type", content = "payload_data", rename_all = "lowercase")]
pub enum Payload {
    PairRequest(PairRequest),
    PairResponse(PairRequest),
    Encrypted(String),
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub device: String,
    pub os: String,
    pub public_key: String,
    pub fingerprint: String,
}


impl Device {
    pub fn new_for_me() -> Self {
        Self {
            device: "foo".into(),
            os: "debian".into(),
            public_key: "no boi".into(),
            fingerprint: "noot".into(),
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRequest {
    pub device: String,
    pub os: String,
    pub public_key: String,
    pub fingerprint: String,
}


impl PairRequest {
    pub fn new_for_me() -> Self {
        Self {
            device: "foo".into(),
            os: "debian".into(),
            public_key: "no boi".into(),
            fingerprint: "noot".into(),
        }
    }
}
