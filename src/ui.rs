extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
extern crate serde_json;
extern crate notify_rust;
extern crate hostname;





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
    ComboBoxText,
    ComboBox,
    ComboBoxExt,
};
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


    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);


    let header_bar = HeaderBar::new();
    let btn_win = window.clone();
    header_bar.add(&{
        let device_dropdown = gtk::MenuButton::new();

        device_dropdown.add(&Button::new_with_label("foo"));

        // device_dropdown.connect_changed(|e| {
        //     println!("combobox clicked{:#?}", e);
        // });

        device_dropdown
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





fn config_new_remote_device(parent: &gtk::Window) {
    let window = gtk::Window::new(gtk::WindowType::Popup);
    window.set_transient_for(parent);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);


    let button = Button::new_with_label("connect");
    button.set_always_show_image(true);
    let window_button = window.clone();
    button.connect_clicked(move |_| window_button.close());



    let header_bar = HeaderBar::new();
    header_bar.add(&button);

    header_bar.set_title("Connection request");
    header_bar.set_show_close_button(false);

    window.set_titlebar(&header_bar);


    window.add(&gtk::Label::new("foobar"));


    window.show_all();
}
