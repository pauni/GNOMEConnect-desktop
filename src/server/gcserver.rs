use gnomeconnect::events;
use gnomeconnect::events::Report;
use hostname::get_hostname;
use serde_json;
use server::devicemanager;
use server::packets;
use server::packets::request;
use server::packets::response;
use std;
use std::io::{Read, Write, BufRead, BufReader,BufWriter};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;




// all connections related stuff here
// device pairing, encryption, etc
// handly of userdata in seperate module






pub fn start_listener_loop(tcp_server: TcpListener)  {

    let mut gcs = GCServer::new();


    for stream in tcp_server.incoming() {
        debug!("TCPconnection established");


        let mut buf_stream = BufReader::new(stream.unwrap());

        // read the data
        let mut data = String::new();
        buf_stream.read_line(&mut data).unwrap();

        debug!("read {:6} bytes", data.len());

        let mut stream = buf_stream.into_inner();

        debug!("{}", data);

        match serde_json::from_str::<packets::TransportPacket>(&data) {
            Err(e) => {
                error!("received malformed package: {}: {}", e, data);
                std::io::stdout().flush();


                stream.write_all (
                    format!("{}: {}", e, data).as_bytes()
                ).unwrap();

                drop(stream);

            },
            Ok(r) => {

                debug!("parsed packet successfully");

                let packet = packets::TransportPacket {
                    src_fingerprint: "noot".into(),
                    dst_fingerprint: r.src_fingerprint.clone(),
                    version: "nope".into(),
                    payload: gcs.process_packet(r).unwrap()
                };

                stream.write_all(serde_json::to_string(&packet).unwrap().as_bytes()).unwrap();

                drop(stream);
            }
        }
    };
}




pub struct ConnectionHandler {
    src_id: String,
    stream: TcpStream,
}


impl ConnectionHandler {
    pub fn new(stream: TcpStream) -> Self {





        Self {
            src_id: "foo".into(),
            stream: stream
        }
    }



    pub fn respond(packet: packets::Payload) {

    }


}
























pub struct GCServer {
    dev_mngr: devicemanager::DeviceManager,
}





impl GCServer {
    fn new() -> Self {
        // let tcp_server = match TcpListener::bind(super::BIND_ADDR) {
        //     Ok(s) => s,
        //     Err(e) => panic!("can't bind to {}: {}", super::BIND_ADDR, e),
        // };
        //
        //
        // thread::spawn(move || {
        //     info!("start listening at {}", super::BIND_ADDR);
        //     start_listener_loop(tcp_server);
        // }).join();


        Self {
            dev_mngr: devicemanager::DeviceManager::new()
        }
    }


    fn process_packet(
        &mut self,
        packet: packets::TransportPacket
    ) -> Option<packets::Payload>
    {
        debug!("received package from {}", packet.src_fingerprint);

        match packet.payload {
            packets::Payload::Pairing(r) => {
                debug!("received Pairrequest from {}", packet.src_fingerprint);


                Some(packets::Payload::Pairing(packets::Pairing {
                    action: packets::PairingAction::Accepted,
                    device: Some(packets::PairInfo {
                        fingerprint: packet.dst_fingerprint,
                        public_key: "foo".into(),
                        os: "macOS".into(),
                        model: "GNOME-shell-3.26".into(),
                        hostname: get_hostname().unwrap()
                    })
                }))
            },

            _ => {
                warn!("packet type not supported");
                None
            }
        }
    }
}
