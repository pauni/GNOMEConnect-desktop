pub mod event;
pub mod transponder;
use serde_json;


pub const BIND_ADDR: &str = "0.0.0.0:4112";


pub fn test_it() {
    let packet = Packet {
        hostname: "oneplus3".into(),
        fingerprint: "aaaded0f-d6ed-41a7-8c17-761360b25297".into(),
        version: "0.0.0.0.1-prealpha".into(),
        payload: PacketType::PairRequest (
            PairRequest {
                os: "macos".into(),
                model: "macpro".into(),
                public_key: "tschuuu".into(),
                fingerprint: "fooobar".into()
        })
    };

    let json_string = serde_json::to_string_pretty(&packet).unwrap();

    println!("{}", json_string);
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub hostname: String,
    pub fingerprint: String,
    pub version: String,
    #[serde(rename = "payload")]
    pub payload: PacketType,
}





// All posible payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "data_type", content = "data")]
pub enum PacketType {
    PairRequest(PairRequest),
    UserData(String),
}




//---------------------------------//
// Available unencrypted datatypes //
//---------------------------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRequest {
    pub os: String,
    pub model: String,
    pub public_key: String,
    pub fingerprint: String,
}





//----------------------//
// Available  datatypes //
//----------------------//
