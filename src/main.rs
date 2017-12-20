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


mod server;
mod config;
mod ui;


use gnomeconnect::events;
use std::net::TcpListener;


pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;





fn main() {
    pretty_env_logger::init().unwrap();
    server::transponder::start();


    let tcp_server = match TcpListener::bind(BIND_ADDR) {
        Ok(s) => s,
        Err(e) => panic!("can't bind to {}: {}", BIND_ADDR, e),
    };


    server::gcserver::start_listener_loop(tcp_server);

    // let gcserver = server::gcserver::GCServer::new(BIND_ADDR);




    std::process::exit(0);


}
