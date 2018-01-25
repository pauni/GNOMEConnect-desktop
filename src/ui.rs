use gio::prelude::*;
use gtk::*;
use gtk;
use gtk::prelude::*;
use std;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;
use gio;




const GENERAL_TAB_ID: &str = "list_tab_general";
const POWER_TAB_ID: &str = "list_tab_power";
const NOTIFICATION_TAB_ID: &str = "list_tab_notifications";
const FILES_TAB_ID: &str = "list_tab_files";
const ACTION_LIST_ID: &str = "action_list";




#[derive(Clone)]
pub struct MainWindow {
	main_window: Window,
	builder: Builder,
}














impl MainWindow {
	pub fn init() -> Self
	{
		if gtk::init().is_err() {
			panic!("Failed to initialize GTK.");
		}


		let builder = Builder::new_from_file("src/main.ui");


		let main_window: Window = builder.get_object("MainWindow").unwrap();


		let general_tab: ListBoxRow = builder.get_object(GENERAL_TAB_ID).unwrap();
		let power_tab: ListBoxRow = builder.get_object(POWER_TAB_ID).unwrap();
		let notifications_tab: ListBoxRow = builder.get_object(NOTIFICATION_TAB_ID).unwrap();
		let files_tab: ListBoxRow = builder.get_object(FILES_TAB_ID).unwrap();
		let action_list: ListBox = builder.get_object(ACTION_LIST_ID).unwrap();
		let btn_test: Button = builder.get_object("btn_test").unwrap();



		general_tab.connect_activate(
			|x| {
				println!("{:#?}", x);
			}
		);

		power_tab.connect_activate(
			|x| {
				println!("{:#?}", x);
			}
		);

		notifications_tab.connect_activate(
			|x| {
				println!("{:#?}", x);
			}
		);

		files_tab.connect_activate(
			|x| {
				println!("{:#?}", x);
			}
		);

		main_window.show_all();

		Self {
			main_window: main_window,
			builder: builder,
		}
	}



	pub fn launch(&self)
	{
		gtk::main();
	}
}






fn register_gapp()
{
	let gtk_application = gtk::Application::new(
		"org.gnomeconnect.gnomeconnect",
		gio::ApplicationFlags::empty(),
	)
			.unwrap();


	if gtk_application.get_is_registered() {
		println!("GApp is registered");
	}
	else {
		println!("GApp not registered");

		match gtk_application.register(None)
		{
			Ok(r) => println!("app registered"),
			Err(e) => println!("registration failed: {}", e),
		}
	}

}
