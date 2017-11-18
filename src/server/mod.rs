pub mod event;
pub mod transponder;

use serde_json;



pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;







pub fn test_it() {
    let packet = Packet {
        sender: "oneplus3".into(),
        fingerprint: "aaaded0f-d6ed-41a7-8c17-761360b25297".into(),
        protocol_version: "0.0.0.0.1-prealpha".into(),
        data: PacketType::PairRequest {
            public_key: "tschuuu".into(),
            fingerprint: "fooobar".into()
        }
    };

    let json_string = serde_json::to_string_pretty(&packet).unwrap();



    println!("{}", json_string);
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub sender: String,
    pub fingerprint: String,
    pub protocol_version: String,
    #[serde(rename = "payload")]
    pub data: PacketType,
}




#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde()]
pub enum PacketType {
    PairRequest {
        public_key: String,
        fingerprint: String
    },
    UserData(String),
}
