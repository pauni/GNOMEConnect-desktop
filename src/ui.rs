extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
extern crate serde_json;
extern crate notify_rust;
extern crate hostname;





use gtk::prelude::*;
use gtk::Builder;
use gtk::Window;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use gio::prelude::*;





pub fn gui() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }



    let builder = Builder::new_from_file("src/main.ui");


    let main_window: Window = builder.get_object("MainWindow").unwrap();



    main_window.show_all();






    gtk::main();
}





fn config_new_remote_device(parent: &gtk::Window) {

}




fn register_gapp() {
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

}
