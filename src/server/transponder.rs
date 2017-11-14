extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
extern crate serde_json;
extern crate notify_rust;


use gtk::prelude::*;
use gtk::{
    ButtonsType,
    DialogFlags,
    MessageDialog,
    MessageType,
    Window,
    WindowType,
    ApplicationWindow,
    HeaderBar,
};

use hostname;
use std::net::UdpSocket;
use std::thread;
use gio::prelude::*;
use serde_json::Value;



static BIND_ADDR: &str = "0.0.0.0:4112";
const BUFFER_SIZE: usize = 65536;






pub fn start() -> Option<()> {
    println!("start discovery service at {}", BIND_ADDR);

    match UdpSocket::bind(BIND_ADDR) {
        Ok(socket) => {
            thread::Builder::new()
                .name("transponder".into())
                .spawn(move || {
                    transponder_loop(socket)
                });

            Some(())
        },
        Err(e) => panic!("binding to {} failed: {}", BIND_ADDR, e)
    }
}




#[derive(Clone, Debug, Serialize)]
struct EchoSignal {
    hostname: String,
    fingerprint: String,
    os: String,
}







fn transponder_loop(udp_sock: UdpSocket) {

    loop {
        let mut buffer = [0; BUFFER_SIZE];
        let (length, remote_addr) = udp_sock.recv_from(&mut buffer).unwrap();

        println!("{:#?}", Vec::from(&buffer[0..length]));


        let echo = EchoSignal {
            hostname: hostname::get_hostname().unwrap(),
            fingerprint: "todo".into(),
            os: "debian".into()
        };



        let send = serde_json::to_string(&echo).unwrap();


        udp_sock.send_to(&send.into_bytes(), remote_addr).unwrap();

        println!("data sent");
    }
}
