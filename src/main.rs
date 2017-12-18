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


mod server;
mod config;
mod ui;


use gnomeconnect::events;



pub const BIND_ADDR: &str = "0.0.0.0:4112";
pub const BUFFER_SIZE: usize = 65536;





fn main() {
    pretty_env_logger::init().unwrap();



    server::transponder::start();

    let event_listener = server::gcserver::start();


    ui::gui();



    std::process::exit(0);






}
