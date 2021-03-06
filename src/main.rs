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


use gtk::prelude::*;
use gtk::{
    Button,
    ButtonsType,
    DialogFlags,
    MessageDialog,
    MessageType,
    Window,
    WindowType,
    ApplicationWindow,
    HeaderBar,
};
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use gio::prelude::*;



pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;






#[derive(Debug, Clone, Serialize, Deserialize)]
struct Foo<T> {
    foo: T
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Bar {
    bar: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Baz {
    baz: String
}






fn main() {
    pretty_env_logger::init().unwrap();
    



    // let packet = server::Packet {
    //     hostname: "oneplus3".into(),
    //     fingerprint: "aaaded0f-d6ed-41a7-8c17-761360b25297".into(),
    //     version: "0.0.0.0.1-prealpha".into(),
    //     payload: server::PacketType::PairRequest (
    //         server::PairRequest {
    //             os: "macos".into(),
    //             model: "macpro".into(),
    //             public_key: "tschuuu".into(),
    //             fingerprint: "fooobar".into()
    //     })
    // };



    // let json_string = serde_json::to_string_pretty(&packet).unwrap();

    // println!("{}", json_string);







    // server::test_it();

    server::transponder::start();



    let event_listener = server::gcserver::start();
    std::process::exit(0);






}
