extern crate gtk;
extern crate gio;
extern crate gnomeconnect;
extern crate serde_json;
extern crate notify_rust;
extern crate hostname;





use gtk::prelude::*;
use gtk::*;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use gio::prelude::*;
use std;




const GENERAL_TAB_ID:      &str  = "list_tab_general";
const POWER_TAB_ID:        &str  = "list_tab_power";
const NOTIFICATION_TAB_ID: &str  = "list_tab_notifications";
const FILES_TAB_ID:        &str  = "list_tab_files";
const ACTION_LIST_ID:      &str  = "action_list";






pub fn gui() {
	if gtk::init().is_err() {
		println!("Failed to initialize GTK.");
		return;
	}


	println!("{}", GENERAL_TAB_ID);

	let builder = Builder::new_from_file("src/main.ui");


	let main_window: Window = builder.get_object("MainWindow").unwrap();



	let general_tab: ListBoxRow       = builder.get_object(GENERAL_TAB_ID).unwrap();
	let power_tab: ListBoxRow         = builder.get_object(POWER_TAB_ID).unwrap();
	let notifications_tab: ListBoxRow = builder.get_object(NOTIFICATION_TAB_ID).unwrap();
	let files_tab: ListBoxRow         = builder.get_object(FILES_TAB_ID).unwrap();
	let action_list: ListBox          = builder.get_object(ACTION_LIST_ID).unwrap();
	let btn_test: Button              = builder.get_object("btn_test").unwrap();



	btn_test.connect_button_press_event(|x, y| {
		println!("{:#?}", x);
		Inhibit(false)
	});

	general_tab.connect_activate(|x| {
		println!("{:#?}", x);
	});

	power_tab.connect_activate(|x| {
		println!("{:#?}", x);
	});

	notifications_tab.connect_activate(|x| {
		println!("{:#?}", x);
	});

	files_tab.connect_activate(|x| {
		println!("{:#?}", x);
	});




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
