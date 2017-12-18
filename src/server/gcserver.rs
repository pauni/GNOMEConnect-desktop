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
use server::packets;
use server::packets::request;
use server::packets::response;



// all connections related stuff here
// device pairing, encryption, etc
// handly of userdata in seperate module





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
        debug!("TCPconnection established");


        let mut buf_stream = BufReader::new(stream.unwrap());

        // read the data
        let mut data = String::new();
        buf_stream.read_line(&mut data).unwrap();

        debug!("read {:6} bytes", data.len());

        let mut stream = buf_stream.into_inner();

        match serde_json::from_str(&data) {
            Err(e) => {
                error!("received malformed package: {}: {}", e, data);

                stream.write_all (
                    "dafuq you wanna say, bro?".as_bytes()
                ).unwrap();
            },
            Ok(r) => {

                let respkt = gcs.process_packet(r);


                match respkt {
                    Some(r) => stream.write_all(serde_json::to_string(&r).unwrap().as_bytes()).unwrap(),
                    None => {}
                };
            }
        };
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


    fn process_packet(
        &mut self,
        packet: packets::TransportPacket
    ) -> Option<packets::TransportPacket>
    {
        debug!("received package from {}", packet.fingerprint);

        match packet.payload {
            packets::Payload::PairRequest(r) => {
                debug!("received Pairrequest from {}", packet.fingerprint);

                // self.dev_mngr.pair_device(r);

                let packet = packets::TransportPacket {
                    fingerprint: "noot".into(),
                    version: "nope".into(),
                    payload: packets::Payload::PairResponse(packets::PairRequest::new_for_me()),
                };

                Some(packet)
            },
            _ => {
                warn!("packet type not supported");
                None
            }
        }
    }
}
