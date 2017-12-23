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


    // ui::gui();


    server::transponder::start();


    let tcp_server = match TcpListener::bind(BIND_ADDR) {
        Ok(s) => s,
        Err(e) => panic!("can't bind to {}: {}", BIND_ADDR, e),
    };


    server::gcserver::start_listener_loop(tcp_server);


    std::process::exit(0);
}
