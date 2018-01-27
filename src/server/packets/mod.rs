pub mod request;
pub mod response;


// use std::io::{Write, BufRead, BufReader, BufWriter};

use hostname::get_hostname;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std;








#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransportHeader {
	pub version: Option<i64>,
	#[serde(rename = "type")]
	pub type_: Action,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag="type", content="data")]
pub enum TransportPackage {
	PairRequest(PairRequest)
}







#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseHeader {
	pub fingerprint: String,
	pub version: Option<i64>,
	pub authorized: bool,
}






#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PairRequest {
	pub shared_secret: String,
	pub device_info: DeviceInfo
}







#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
	Pairing,
	Encrypted,
}




#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
	pub device_name: String,
	pub fingerprint: String,
	pub version: f64,
	pub os: String,
}




#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PairingResponse {
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
