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
extern crate env_logger;
extern crate openssl;

mod server;
mod config;
mod ui;
mod rsa;

use gnomeconnect::events;
use server::devicemanager;
use server::packets;
use server::packets::Action;
use server::gcserver::StreamHandler;
use serde_json::to_string as to_json;

pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;
pub const SERVER_QUEUE_CAPACITY: usize = 3;







fn main() {
	pretty_env_logger::init().unwrap();

	let device_manager = devicemanager::DeviceManager::new();

	server::transponder::start();

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
