use serde_json;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;






pub fn start() {
    let (events_tr, events_rx) = mpsc::channel::<Report>();

    let tcp_server = match TcpListener::bind(super::BIND_ADDR) {
        Ok(s) => s,
        Err(e) => panic!("can't bind to {}: {}", super::BIND_ADDR, e),
    };


    thread::spawn(move || {
        println!("start listening at {}", super::BIND_ADDR);
        listener_loop(tcp_server);
    }).join();

    println!("event server thread spawned");
}





fn listener_loop(tcp_server: TcpListener) {
    for conn in tcp_server.incoming() {
        let mut data = String::new();
        let mut stream = conn.unwrap();

        stream.read_to_string(&mut data);

        let package = match Packet::from_string(data) {
            Err(e) => {
                eprintln!("received malformed package: {}", e);
                continue;
            },
            Ok(r) => r
        };

        process_event(package);
    };
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Types {
    PairRequest {
        public_key: String
    },
    UserData
}




#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Packet {
    pub fingerprint: String,
    pub sender: String,
    pub version: String,
    // the encrypted payload
    pub payload: String
}




impl Packet {
    pub fn from_string(string: String) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str::<Packet>(&string)

        // match serde_json::from_str::<Packet>(&string) {
        //     Ok(r) => Ok(r),
        //     Err(e) => Err(e)
        // }
    }
}











fn process_event(report: Packet) {

    println!("{:#?}", report);
}
