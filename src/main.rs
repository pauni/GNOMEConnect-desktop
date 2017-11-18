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



static BIND_ADDR: &str = "0.0.0.0:4112";
const BUFFER_SIZE: usize = 65536;





fn main() {

    server::test_it();

    server::transponder::start();



    let event_listener = server::event::start();

    std::process::exit(0);





}
