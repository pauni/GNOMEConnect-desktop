use gnomeconnect::events;
use gnomeconnect::events::Report;
use hostname::get_hostname;
use packets::TransportPacket;
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


const PROTOCOL_VERSION: i64 = 45;





pub fn start_listener_loop(tcp_server: TcpListener)  {

    let mut gcs = GCServer::new();


    for stream in tcp_server.incoming() {

        debug!("TCPconnection established");


    };
}




pub struct ConnectionHandler {
    stream: TcpStream,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream) -> Self{
        Self {
            stream: stream
        }
    }


    pub fn receive_packet(mut self) -> Option<TransportPacket> {
        let data = self.read_line();

        match serde_json::from_str::<TransportPacket>(&data) {
            Err(e) => {
                error!("received malformed package: {}: {}", e, data);
                None
            },
            Ok(r) => Some(r)
        }
    }




    pub fn send_packet(&mut self, payload: packets::Payload) -> Option<()> {
        debug!("parsed packet successfully");

        warn!("Todo: devicemanager integration");


        let packet = packets::TransportPacket {
            src_fingerprint: "noot".into(),
            dst_fingerprint: "foo".into(),
            version: PROTOCOL_VERSION,
            payload: payload
        };

        match self.stream.write_all(serde_json::to_string(&packet).unwrap().as_bytes()) {
            Ok(_) => Some(()),
            Err(e) => {
                error!("Failes to write to stream: {}", e);
                None
            }
        }
    }


    fn read_line(mut self) -> String {
        let mut buf_stream = BufReader::new(self.stream);

        // read the data
        let mut data = String::new();
        buf_stream.read_line(&mut data).unwrap();

        debug!("read {:6} bytes", data.len());

        // hand back the stream

        data
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
            dev_mngr: devicemanager::DeviceManager::init()
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
                None
            },

            _ => {
                warn!("packet type not supported");
                None
            }
        }
    }
}
