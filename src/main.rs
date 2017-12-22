extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate notify_rust;
extern crate hostname;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate openssl;



mod server;
mod config;
mod ui;
mod rsa;


use gnomeconnect::events;
use std::net::TcpListener;
use std::io;
use server::packets;
use server::packets::Payload;
use server::packets::TransportPacket;


pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;








fn main() {
    pretty_env_logger::init().unwrap();

    let pairing = Payload::Pairing( packets::PairingStep::KeyRx("foo".into()) );


    let packet = TransportPacket {
        src_fingerprint: "foo".into(),
        dst_fingerprint: "bar".into(),
        version: 45,
        payload: pairing
    };



    let wire = serde_json::to_string_pretty(&packet).unwrap();






    server::devicemanager::DeviceManager::init();






}







// fn main() {
//     pretty_env_logger::init().unwrap();
//
//
//     ui::gui();
//
//
//     server::transponder::start();
//
//
//     let tcp_server = match TcpListener::bind(BIND_ADDR) {
//         Ok(s) => s,
//         Err(e) => panic!("can't bind to {}: {}", BIND_ADDR, e),
//     };
//
//
//     server::gcserver::start_listener_loop(tcp_server);
//
//     // let gcserver = server::gcserver::GCServer::new(BIND_ADDR);
//
//
//
//
//     std::process::exit(0);
//
//
// }
