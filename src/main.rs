#[macro_use]extern crate log;
#[macro_use]extern crate serde_derive;
#[macro_use]extern crate serde_json;
extern crate base64;
extern crate env_logger;
extern crate gio;
extern crate gnomeconnect;
extern crate gtk;
extern crate hostname;
extern crate notify_rust;
extern crate openssl_sys;
extern crate openssl;
extern crate pretty_env_logger;
extern crate crypto;
extern crate serde;
extern crate clap;

mod config;
mod rsa;
mod server;
mod ui;

use gnomeconnect::events;
use serde_json::to_string as to_json;
use server::devicemanager;
use server::gcserver::StreamHandler;
use server::packets;
use server::packets::Action;
use clap::App;
use clap::Arg;
use clap::SubCommand;

pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;
pub const SERVER_QUEUE_CAPACITY: usize = 3;







fn main() {
	pretty_env_logger::init().unwrap();


	let matches = App::new("My Super Program")
		.arg(Arg::with_name("gui")
			.long("gui")
			.help("Start the GUI for GNOMEConnect. THIS IS BLOCKING")
			.takes_value(false))
		.arg(Arg::with_name("transponder")
			.long("transponder")
			.help("Start the UDP transponder")
			.takes_value(false))

		.get_matches();


	println!("{:#?}", matches);




	let device_manager = devicemanager::DeviceManager::new();
	let public_key = device_manager.get_public_key();


	println!("{}", public_key);

	if matches.is_present("transponder") {
		server::transponder::start(device_manager.get_public_key());
	}


	if matches.is_present("gui") {
		ui::MainWindow::init().launch();
	}











	// let gui = ui::MainWindow::init();
	// gui.launch();


	let server = server::gcserver::spawn_server(BIND_ADDR, SERVER_QUEUE_CAPACITY)
		.expect("can't spwn server. Inspect previeous errors");


	for mut connection in server.into_iter() {
		info!("connection received");
		debug!("Connection parameters: {}", connection.remote_ip());
		debug!("    remote address: {}", connection.remote_ip());
		debug!("    remote id     : {}", connection.remote_id());
		debug!("    type          : {:?}", connection.action());


		let res = packets::ResponseHeader {
			fingerprint: "foo".into(),
			version: Some(1),
			authorized: device_manager.is_paired(connection.remote_id())
		};


		debug!("answer");
		debug!("    {}", to_json(&res).unwrap());
		connection.write_line(to_json(&res).unwrap());




		match connection.action() {
			Action::Pairing => pairing(connection),
			Action::Encrypted => warn!("encrypted"),
		};



		std::process::exit(0);
	}





	std::process::exit(0);
}


fn pairing(sh: StreamHandler) {
	let data = sh.read_line();


	debug!("{}", data);


}
