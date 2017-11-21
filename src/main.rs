extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate notify_rust;
extern crate hostname;


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





fn main() {

    let packet = server::Packet {
        hostname: "oneplus3".into(),
        fingerprint: "aaaded0f-d6ed-41a7-8c17-761360b25297".into(),
        version: "0.0.0.0.1-prealpha".into(),
        payload: server::PacketType::PairRequest (
            server::PairRequest {
                os: "macos".into(),
                model: "macpro".into(),
                public_key: "tschuuu".into(),
                fingerprint: "fooobar".into()
        })
    };



    let json_string = serde_json::to_string_pretty(&packet).unwrap();

    println!("{}", json_string);







    server::test_it();

    server::transponder::start();



    let event_listener = server::event::start();
    std::process::exit(0);






}
