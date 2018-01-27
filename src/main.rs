#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
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
extern crate rand;

mod config;
mod rsa;
mod server;
mod ui;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use serde::Serialize;
use serde_json::to_string as to_json;
use server::devicemanager;
use server::gcserver::StreamHandler;
use server::packets;
use server::packets::Action;

pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;
pub const SERVER_QUEUE_CAPACITY: usize = 3;




// CLI-constants
const CLI_GEN_PACKETS: &str = "genpackets";




fn main()
{
	pretty_env_logger::init().unwrap();


	let matches = App::new("My Super Program")
		.arg(
			Arg::with_name("gui")
				.long("gui")
				.help("Start the GUI for GNOMEConnect. THIS IS BLOCKING")
				.takes_value(false)
		)
		.arg(
			Arg::with_name("no-transponder")
				.long("no-transponder")
				.help("Disable the UDP transponder")
				.takes_value(false)
		)
		.subcommand(
			App::new(CLI_GEN_PACKETS)
				.help("Generate Package for debugging")
				.arg(
					Arg::with_name("pairing")
						.long("pairing")
						.help("generate pairing packet")
						.takes_value(false)
				)
		)
		.subcommand(
			App::new("debug")
				.subcommand(App::new("add-device"))
				.subcommand(
					App::new("remove-device").arg(
						Arg::with_name("name")
							.long("name")
							.help("remove a device")
							.takes_value(true)
							.required(true)
					)
				)
				.subcommand(App::new("private-key"))
		)
		.get_matches();




	if let Some(sub_m) = matches.subcommand_matches(CLI_GEN_PACKETS) {
		println!("generate packets");
		generate_packages(sub_m.clone());
		std::process::exit(0);
	}

	if let Some(sub_m) = matches.subcommand_matches("debug") {
		match sub_m.subcommand()
		{
			("add-device", Some(_)) => {
				devicemanager::DeviceManager::new().add_device(
					devicemanager::Device::new(
						"debug-fingerprint".to_string(),
						"debug-hostname".to_string(),
						"debug-model".to_string(),
						"debug-Os".to_string(),
						"debug-publickey".as_bytes().to_vec(),
						"debug-sharedsecret".as_bytes().to_vec(),
					)
				);
			}
			("remove-device", Some(args)) => {
				let dev_name = args.value_of("name").unwrap();
				println!("removing device {}", dev_name);

				let mut dm = devicemanager::DeviceManager::new();

				match dm.remove_by_hostname(dev_name.to_string())
				{
					None => println!("device not there"),
					Some(_) => println!("device removed"),
				}
			}
			("private-key", Some(_)) => {
				let mut dm = devicemanager::DeviceManager::new();
				let private_key = String::from_utf8(
					dm.get_rsa().private_key_to_pem().unwrap()
				).unwrap();

				println!("{}", private_key);
			}
			(_, _) => error!("command not found"),
		}


		std::process::exit(0);
	}




	let device_manager = devicemanager::DeviceManager::new();
	let public_key = device_manager.get_public_key();


	println!("{}", String::from_utf8(public_key).unwrap());

	if !matches.is_present("no-transponder") {
	}

	server::transponder::start(device_manager.get_public_key());

	if matches.is_present("gui") {
		ui::MainWindow::init().launch();
	}












	// let gui = ui::MainWindow::init();
	// gui.launch();


	let gcserver = server::gcserver::GCServer::spawn_server(BIND_ADDR)
		.expect("can't spwn server. Inspect previous errors");


	for mut connection in gcserver {
		info!("connection received");
		debug!("Connection parameters: {}", connection.remote_ip());
		debug!("    remote address: {}", connection.remote_ip());


		println!("{:#?}", connection.recv_package());

		println!("{:#?}", connection.recv_package());





		println!("{:#?}", connection);

	}

	std::process::exit(0);
}




fn generate_packages(matches: clap::ArgMatches)
{
	if matches.is_present("pairing") {
		let example = server::packets::TransportPackage::PairRequest (
			packets::PairRequest::default()
		);

		print_packet(example);
	}





	fn print_packet<T: Serialize>(packet: T)
	{
		let string = serde_json::to_string_pretty(&packet).unwrap();

		println!("{}", string);
	}

}






fn handle_connection(conn: StreamHandler) {
	info!("start stream handler");
	println!("{:#?}", conn);


	// if !conn.is_paired() {
	// 	return;
	// }



}
