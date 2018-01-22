pub mod request;
pub mod response;


// use std::io::{Write, BufRead, BufReader, BufWriter};
use hostname::get_hostname;
use std;
use serde::Serialize;
use serde::Deserialize;
use serde::de::DeserializeOwned;








#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransportHeader {
    pub fingerprint: String,
    pub version: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Action,
}







#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseHeader {
    pub fingerprint: String,
    pub version: Option<i64>,
    pub authorized: bool,
}






#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pairing {
    pub message: String,
    pub signature: String,
    pub public_key: String,
    pub fingerprint: String,
}



impl Pairing {
    pub fn gen_example() -> Self {
        Self {
            message: "<encrypted message>".to_string(),
            signature: "<DSA-signature>".to_string(),
            public_key: "<public key>".to_string(),
            fingerprint: "<fingerprint>".to_string(),
        }
    }
}














#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Pairing,
    Encrypted,
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralDeviceInformation {
    pub device: String,
    pub os: String,
    pub public_key: String,
    pub fingerprint: String,
}



impl GeneralDeviceInformation {
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
#[serde(rename_all = "snake_case")]
pub enum PairingAction {
    Request,
    Denied,
    Accepted,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PairInfo {
    pub fingerprint: String,
    pub public_key: String,
    pub os: String,
    pub model: String,
    pub hostname: String,
}



#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(tag = "step", content = "data", rename_all = "snake_case")]
pub enum PairingStep {
    #[serde(rename = "1")]
    KeyRx(String),

    #[serde(rename = "2")]
    KeyTx(String),

    #[serde(rename = "3")]
    InformationRx(String),

    #[serde(rename = "4")]
    InformationTx(String),

    #[serde(rename = "1")]
    KeyExchange_(String),

    #[serde(rename = "1")]
    KeyExchange__(String),
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PairingHello {
    pub fingerprint: String,
    pub public_key: String,
    pub os: String,
    pub model: String,
    pub hostname: String,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PairingKeyExchange {
    pub public_key: String,
}
