extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate notify_rust;
extern crate hostname;


mod transponder;
mod config;


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

    let (events_tr, events_rx) = mpsc::channel::<Report>();


    thread::spawn(move || {
        println!("start listening at {}", BIND_ADDR);


        for conn in TcpListener::bind(BIND_ADDR).unwrap().incoming() {
            let mut data = String::new();
            let mut stream = conn.unwrap();

            stream.read_to_string(&mut data);

            match serde_json::from_str::<Vec<events::Report>>(&data) {
                Err(e) => {
                    eprintln!("json parsing failed: {}", e);
                    stream.write_all(format!("{}", e).as_bytes());
                },
                Ok(event_list) => {
                    for event in event_list {
                        events_tr.send(event);
                    }
                    stream.write_all("OK".as_bytes());
                }
            }
        };
    });


    transponder::start();







    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }


    let gtk_application = gtk::Application::new(
        "org.gnomeconnect.gnomeconnect",
        gio::ApplicationFlags::empty()
    ).unwrap();


    if gtk_application.get_is_registered() {
        println!("GApp is registered");
    }
    else {
        println!("GApp not registered");

        match gtk_application.register(None) {
            Ok(r) => println!("app registered"),
            Err(e) => println!("registration failed: {}", e)
        }
    }


    // Window shit


    let window = ApplicationWindow::new(&gtk_application);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);


    let header_bar = HeaderBar::new();
    header_bar.add(&{
        let button = Button::new_with_mnemonic("connect");
        button.set_always_show_image(true);
        button.set_label("Connect");
        button.set_name(".suggested-action");
        button
    });

    header_bar.set_title("GNOMEConnect");
    header_bar.set_show_close_button(true);

    window.set_titlebar(&header_bar);



    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(move |_| {

        notify_rust::Notification::new()
            .summary("Battery nearly full")
            .body("Battery level is <b>98%</b>. You're ready to go!")
            .icon("whatsapp")
            .show()
            .unwrap();

    });

    gtk::main();
}





fn config_new_remote_device() {
    let window = gtk::Window::new(gtk::WindowType::Popup);

    



}








fn process_event(report: Report) {

    println!("{:#?}", report);
}
