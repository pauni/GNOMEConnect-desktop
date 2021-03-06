use serde_json;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use server::gcserver;






#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub hostname: String,
    pub os: String,
    pub model: String,
    pub public_key: String,
    pub fingerprint: String,
}


impl Device {
    pub fn new(
        hostname: String,
        model: String,
        os: String,
        public_key: String,
        fingerprint: String
    ) -> Self {
        Self {
            hostname: hostname,
            model: model,
            os: os,
            public_key: public_key,
            fingerprint: fingerprint
        }
    }
}




impl From<gcserver::PairRequest> for Device {
    fn from(pr: gcserver::PairRequest) -> Self {
        Device::new(
            pr.hostname,
            pr.device,
            pr.os,
            pr.public_key,
            pr.fingerprint
        )
    }
}


pub struct DeviceManager {
    devices: HashMap<String, Device>,
}



impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new()
        }
    }


    pub fn device_paired(&self, fingerprint: String) -> bool {
        self.devices.contains_key(&fingerprint)
    }

    pub fn pair_device(&mut self, pr: super::gcserver::PairRequest) {
        info!("pair device {}", pr.fingerprint);
        
        self.devices.insert(
            pr.clone().fingerprint,
            Device::from(pr)
        );
    }



    pub fn get_device(&self, fingerprint: String) -> Option<&Device> {
        self.devices.get(&fingerprint)
    }
}
