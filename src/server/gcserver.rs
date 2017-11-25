use serde_json;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::io::{
    Read,
    Write,
    BufRead,
    BufReader,
    BufWriter
};
use std::sync::mpsc;
use std::thread;
use server::devicemanager;
use hostname::get_hostname;
use std;




static PROTOCOL_VERSION: &str = "0.0.1-alpha";




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

}






fn start_listener_loop(tcp_server: TcpListener) {

    let mut gcs = GCServer::new();


    for stream in tcp_server.incoming() {
        let mut buf_stream = BufReader::new(stream.unwrap());

        let mut data = String::new();
        buf_stream.read_line(&mut data).unwrap();


        let packet = match Packet::from_string(data.clone()) {
            Err(e) => {
                error!("received malformed package: {}: {}", e, data);

                buf_stream.into_inner().write_all (
                    "dafuq you wanna say, bro?".as_bytes()
                ).unwrap();

                continue;
            },
            Ok(r) => r
        };



        let respkt = gcs.process_packet(packet);

        let mut stream = buf_stream.into_inner();

        match respkt {
            Some(r) => stream.write_all(serde_json::to_string(&r).unwrap().as_bytes()).unwrap(),
            None => continue
        };

        drop(stream);
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


    fn process_packet(&mut self, packet: Packet) -> Option<Packet> {
        info!("received package from {}", packet.fingerprint());


        match packet.payload {
            PacketType::PairRequest(r) => {
                debug!("received Pairrequest from {}", packet.fingerprint);

                self.dev_mngr.pair_device(r);

                Some(Packet::new(
                    PacketType::PairRequest(
                        PairRequest::new_for_me()
                    )
                ))
            },
            _ => {
                warn!("packet type not supported");
                None
            }
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub fingerprint: String,
    pub version: String,
    #[serde(rename = "payload")]
    pub payload: PacketType,
}



impl Packet {

    fn new(payload: PacketType) -> Self {
        Self {
            fingerprint: "footprint".into(),
            version: PROTOCOL_VERSION.into(),
            payload: payload,
        }
    }


    pub fn from_string(string: String) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str::<Packet>(&string)
        // match serde_json::from_str::<Packet>(&string)
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
    pub device: String,
    pub os: String,
    pub public_key: String,
    pub fingerprint: String,
}



impl PairRequest {
    fn new_for_me() -> Self {
        Self {
            hostname: get_hostname().unwrap(),
            device: "foo".into(),
            os: "debian".into(),
            public_key: "no boi".into(),
            fingerprint: "noot".into(),
        }
    }
}
