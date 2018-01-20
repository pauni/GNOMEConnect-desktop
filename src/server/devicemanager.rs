use base64;
use crypto::digest::Digest;
use crypto::md5;
use gnomeconnect::events;
use gnomeconnect::events::Report;
use openssl::hash::MessageDigest;
use openssl::pkey;
use openssl::pkey::PKey;
use openssl::rsa;
use openssl::sign;
use openssl::sign::Signer;
use openssl::sign::Verifier;
use serde_json;
use server::gcserver;
use server::packets;
use server::packets::request;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;




const KEY_LENGTH: u32 = 4096;


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




impl From<packets::request::PairRequest> for Device {
	fn from(pr: packets::request::PairRequest) -> Self {
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

	private_key: String,
	public_key: String,
	priv_pem: Vec<u8>,
	pub_pem: Vec<u8>,
}



impl DeviceManager {
	pub fn new() -> Self {
		debug!("generate key");
		let rsa_key = rsa::Rsa::generate(KEY_LENGTH).unwrap();
		debug!("keys generated");


		let device_map: HashMap<String, Device> = HashMap::new();


		Self {
			priv_pem: rsa_key.private_key_to_pem().unwrap(),
			pub_pem: rsa_key.public_key_to_pem().unwrap(),
			devices: device_map,
			private_key: "noot".into(),
			public_key: "noot".into()
		}
	}


	fn rsa(&self) -> rsa::Rsa {
		rsa::Rsa::private_key_from_pem(&self.priv_pem).unwrap()
	}


	pub fn get_public_key(&self) -> String {
		String::from_utf8(self.pub_pem.clone()).unwrap()
	}


	pub fn get_public_key_fingerprint(&self) -> String {
		let mut hasher = md5::Md5::new();
		hasher.input_str(&self.get_public_key());

		hasher.result_str()
	}




	pub fn is_paired(&self, fingerprint: String) -> bool {
		self.devices.contains_key(&fingerprint)
	}



	pub fn get_device(&self, fingerprint: String) -> Option<&Device> {
		self.devices.get(&fingerprint)
	}




	pub fn pair_device(&self, pairing: packets::Pairing) -> Option<()> {
		// https://docs.rs/openssl/0.9.23/openssl/sign/index.html

		let rsa = rsa::Rsa::private_key_from_pem(&self.priv_pem).unwrap();
		let keypair = pkey::PKey::from_rsa(rsa).unwrap();


		let signature = base64::decode(&pairing.signature).unwrap();
		let message = pairing.message;

		let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
		verifier.update(&message.as_bytes());


		verifier.verify(&signature);



		None
	}



}
