use serde_json;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use server::devicemanager;






pub fn start() {
    let (events_tr, events_rx) = mpsc::channel::<Report>();

    let tcp_server = match TcpListener::bind(super::BIND_ADDR) {
        Ok(s) => s,
        Err(e) => panic!("can't bind to {}: {}", super::BIND_ADDR, e),
    };


    thread::spawn(move || {
        info!("start listening at {}", super::BIND_ADDR);
        start_listener_loop(tcp_server);
    }).join();

    println!("event server thread spawned");
}






fn start_listener_loop(tcp_server: TcpListener) {

    let mut gcs = GCServer::new();


    for stream in tcp_server.incoming() {
        let mut data = String::new();
        stream.unwrap().read_to_string(&mut data);


        let packet = match Packet::from_string(data.clone()) {
            Err(e) => {
                error!("received malformed package: {}: {}", e, data);
                continue;
            },
            Ok(r) => r
        };

        gcs.process_packet(packet)

    };
}


pub struct GCServer {
    dev_mngr: devicemanager::DeviceManager,
}



impl GCServer {

    fn new() -> Self {
        Self {
            dev_mngr: devicemanager::DeviceManager::new()
        }
    }


    fn process_packet(&mut self, packet: Packet) {
        info!("received package from {}", packet.hostname());


        match packet.payload {
            PacketType::PairRequest(r) => {
                debug!("received Pairrequest from {}", packet.hostname);

                self.dev_mngr.pair_device(r);

            },
            _ => warn!("packet type not supported")
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub hostname: String,
    pub fingerprint: String,
    pub version: String,
    #[serde(rename = "payload")]
    pub payload: PacketType,
}


impl Packet {
    pub fn from_string(string: String) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str::<Packet>(&string)

        // match serde_json::from_str::<Packet>(&string) {
        //     Ok(r) => Ok(r),
        //     Err(e) => Err(e)
        // }
    }

    fn hostname(&self) -> String {
        self.hostname.clone()
    }


    fn fingerprint(&self) -> String {
        self.fingerprint.clone()
    }
}






//----------------------//
// All posible payloads //
//----------------------//

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "payload_type", content = "payload_data", rename_all = "lowercase")]
pub enum PacketType {
    PairRequest(PairRequest),
    UserData(String),
}





//---------------------------------//
// Available unencrypted datatypes //
//---------------------------------//


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRequest {
    pub hostname: String,
    pub model: String,
    pub os: String,
    pub public_key: String,
    pub fingerprint: String,
}
