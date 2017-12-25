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
extern crate openssl;

mod server;
mod config;
mod ui;
mod rsa;

use gnomeconnect::events;
use server::devicemanager;
use server::packets;
use server::packets::Action;

pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;
pub const SERVER_QUEUE_CAPACITY: usize = 3;







fn main() {
	pretty_env_logger::init().unwrap();

	let device_manager = devicemanager::DeviceManager::new();


	// ui::gui();

	server::transponder::start();


	let server = server::gcserver::spawn_server(BIND_ADDR, SERVER_QUEUE_CAPACITY)
		.expect("can't spwn server. Inspect previeous errors");


	for connection in server.into_iter() {
		debug!("Received connection:");
		debug!("    remote address: {}", connection.remote_ip());
		debug!("    remote id     : {}", connection.remote_id());
		debug!("    type          : {:?}", connection.action());



		if !device_manager.is_paired(connection.remote_id()) {
			error!("device is not paired");
			error!("dropping connection");
		}
	}





	std::process::exit(0);
}
