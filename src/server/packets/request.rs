
use hostname::get_hostname;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std;
use std::io::{BufRead, BufReader, BufWriter, Write};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload_type", content = "payload_data", rename_all = "lowercase")]
pub enum RequestType {
	PairRequest(PairRequest),
	UserData(String),
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRequest {
	pub hostname: String,
	pub device: String,
	pub os: String,
	pub public_key: String,
	pub fingerprint: String,
}
