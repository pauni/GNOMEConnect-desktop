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
};
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use gio::prelude::*;
use std::fs;
use std::io::prelude::*;





const CONFIG_FILE: &str = "./config.json";




pub fn load_config() -> Option<Config> {
    let config_file = match fs::File::open(CONFIG_FILE) {
        Ok(r) => r,
        Err(e) => return None,
    };

    let config: Config = match serde_json::from_reader(config_file) {
        Err(e) => panic!("failed to parse config: {}", e),
        Ok(r) => r
    };

    Some(config)
}










#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub fingerprint: String,
    pub public_key: String,
    pub private_key: String,
    pub remote_devices: Vec<RemoteDeviceConfig>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteDeviceConfig {
    pub fingerprint: String,
    pub public_key: String,
    pub hostname: String,
    pub os: String
}
