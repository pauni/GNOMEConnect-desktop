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
use serde::Serialize;

pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;
pub const SERVER_QUEUE_CAPACITY: usize = 3;




// CLI-constants
const CLI_GEN_PACKETS: &str = "genpackets";




fn main() {
    pretty_env_logger::init().unwrap();


    let matches = App::new("My Super Program")
        .arg(Arg::with_name("gui")
                 .long("gui")
                 .help("Start the GUI for GNOMEConnect. THIS IS BLOCKING")
                 .takes_value(false))
        .arg(Arg::with_name("no-transponder")
                 .long("no-transponder")
                 .help("Disable the UDP transponder")
                 .takes_value(false))
        .subcommand(App::new(CLI_GEN_PACKETS)
                        .help("Generate Package for debugging")
                        .arg(Arg::with_name("pairing")
                                 .long("pairing")
                                 .help("generate pairing packet")
                                 .takes_value(false)))
        .subcommand(App::new("debug")
                        .subcommand(App::new("add-device"))
                        .subcommand(App::new("remove-device").arg(Arg::with_name("name")
                                                                      .long("name")
                                                                      .help("remove a device",)
                                                                      .takes_value(true)
                                                                      .required(true))))
        .get_matches();




    if let Some(sub_m) = matches.subcommand_matches(CLI_GEN_PACKETS) {
        println!("generate packets");
        generate_packages(sub_m.clone());
        std::process::exit(0);
    }

    if let Some(sub_m) = matches.subcommand_matches("debug") {
        match sub_m.subcommand() {
            ("add-device", Some(_)) => {
                devicemanager::DeviceManager::new()
                    .add_device(devicemanager::Device::new("debug-fingerprint".to_string(),
                                                           "debug-hostname".to_string(),
                                                           "debug-device".to_string(),
                                                           "debug-Os".to_string(),
                                                           "debug-fingerprint".to_string()));
            }
            ("remove-device", Some(args)) => {
                let dev_name = args.value_of("name").unwrap();
                println!("removing device {}", dev_name);

                let mut dm = devicemanager::DeviceManager::new();

                match dm.remove_by_hostname(dev_name.to_string()) {
                    None => println!("device not there"),
                    Some(_) => println!("device removed"),
                }
            }
            (_, _) => error!("command not found"),
        }


        std::process::exit(0);
    }




    let device_manager = devicemanager::DeviceManager::new();
    let public_key = device_manager.get_public_key();


    println!("{}", public_key);

    if !matches.is_present("transponder") {
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






        // println!("{:#?}", connection);

    }





    std::process::exit(0);
}














fn generate_packages(matches: clap::ArgMatches) {
    if matches.is_present("pairing") {
        let example = server::gcserver::Package {
            what: server::gcserver::Type::PairRequest(packets::Pairing::gen_example()),
        };

        print_packet(example);
    }





    fn print_packet<T: Serialize>(packet: T) {
        let string = serde_json::to_string_pretty(&packet).unwrap();

        println!("{}", string);
    }

}
