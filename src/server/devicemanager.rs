use base64;
use crypto::digest::Digest;
use openssl::hash::MessageDigest;
use openssl::pkey;
use openssl::rsa;
use openssl::sha;
use openssl::sign::Verifier;
use serde_json;
use server::packets;
use std::collections::HashMap;
use std::fs;
use std::path;
use std::io::Write;




const KEY_LENGTH: u32 = 4096;
const DEVICE_SAVE_FILE: &str = "./devices.json";



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
	pub hostname: String,
	pub os: String,
	pub model: String,
	pub fingerprint: String,
	pub public_key: Vec<u8>,
	pub shared_secret: Vec<u8>,
}


impl Device {
	pub fn new(
		fingerprint: String,
		hostname: String,
		model: String,
		os: String,
		public_key: Vec<u8>,
		shared_secret: Vec<u8>,
	) -> Self
	{
		Self {
			hostname: hostname,
			model: model,
			os: os,
			public_key: public_key,
			fingerprint: fingerprint,
			shared_secret: shared_secret,
		}
	}


	fn rsa_key(&self) -> rsa::Rsa<pkey::Public>
	{
		rsa::Rsa::public_key_from_pem(&self.public_key).unwrap()
	}
}







#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceManager {
	priv_pem: Vec<u8>,
	pub_pem: Vec<u8>,
	devices: HashMap<String, Device>,
}



impl DeviceManager {
	pub fn load_from_default() -> Self
	{
		info!("loading keys and devices from disk");
		let fd = fs::File::open(DEVICE_SAVE_FILE).unwrap();

		let dm: Self = serde_json::from_reader(fd).expect("failed to read saved devices from disk");

		dm
	}




	pub fn new() -> Self
	{
		match path::Path::new(DEVICE_SAVE_FILE).exists()
		{
			true => Self::load_from_default(),
			false => Self::init(),
		}
	}


	// priv_pem: Vec<u8>, pub_pem: Vec<u8>, devices: HashMap<String, Device>
	pub fn init() -> Self
	{
		info!("initialize new keys");
		debug!("generate key");
		let rsa_key = rsa::Rsa::generate(KEY_LENGTH).unwrap();
		debug!("keys generated");


		let device_map: HashMap<String, Device> = HashMap::new();


		let dm = Self {
			priv_pem: rsa_key.private_key_to_pem().unwrap(),
			pub_pem: rsa_key.public_key_to_pem().unwrap(),
			devices: device_map,
		};

		dm.save_state();

		dm
	}


	pub fn rsa(&self) -> rsa::Rsa<pkey::Private>
	{
		rsa::Rsa::private_key_from_pem(&self.priv_pem).unwrap()
	}


	pub fn get_public_key(&self) -> Vec<u8>
	{
		self.pub_pem.clone()
	}


	pub fn get_public_key_fingerprint(&self) -> Vec<u8>
	{
		sha::sha256(&self.get_public_key()).to_vec()
	}




	pub fn is_paired(&self, fingerprint: String) -> bool
	{
		self.devices.contains_key(&fingerprint)
	}



	pub fn get_device(&self, fingerprint: String) -> Option<&Device>
	{
		self.devices.get(&fingerprint)
	}





	pub fn pair_device(&self, pairing: packets::PairRequest) -> Option<()>
	{
		// https://docs.rs/openssl/0.9.23/openssl/sign/index.html


		unimplemented!();

		// let rsa = rsa::Rsa::private_key_from_pem(&self.priv_pem).unwrap();
		// let keypair = pkey::PKey::from_rsa(rsa).unwrap();
		//
		//
		// let signature = base64::decode(&pairing.signature).unwrap();
		// let message = pairing.message;
		//
		// let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
		// verifier.update(&message.as_bytes());
		//
		//
		// verifier.verify(&signature);
		//
		//
		//
		// None
	}


	pub fn get_rsa(&self) -> rsa::Rsa<pkey::Private>
	{
		rsa::Rsa::private_key_from_pem(&self.priv_pem)
			.expect("something went from while restoring private key")
	}



	pub fn load_device_with_fingerprint(&self, fingerprint: String) -> Option<Device> {
		self.devices.get(&fingerprint).map(|x| {
			x.to_owned()
		})
	}



	pub fn add_device(&mut self, device: Device)
	{
		self.devices.insert(device.fingerprint.clone(), device);
		self.save_state();
	}


	pub fn remove_by_device(&mut self, device: Device) -> Option<()>
	{
		let res = self.devices
			.remove(&device.fingerprint)
			.map(|_| Some(()))
			.unwrap();


		self.save_state();

		res
	}

	pub fn remove_by_fingerprint(&mut self, fingerprint: String) -> Option<()>
	{
		let res = self.devices.remove(&fingerprint).map(|_| Some(())).unwrap();


		self.save_state();

		res
	}


	pub fn remove_by_hostname(&mut self, name: String) -> Option<()>
	{
		for (fp, device) in self.devices.clone() {
			if device.hostname == name {
				return self.remove_by_fingerprint(fp);
			}
		}



		self.save_state();

		None
	}



	pub fn save_state(&self)
	{
		debug!("{:#?}", self);

		let fd = fs::File::create(DEVICE_SAVE_FILE).unwrap();

		serde_json::to_writer_pretty(fd, &self).expect("failed to save devices to disk");
	}




	pub fn decrypt_asym(&self, data: Vec<u8>) -> Option<Vec<u8>>
	{
		let mykey = self.get_rsa();


		let mut decrypted: Vec<u8> = Vec::new();

		let mut fd = fs::File::create("data").unwrap();

		fd.write_all(&data);

		let result = mykey.private_decrypt(
			&data,
			decrypted.as_mut_slice(),
			rsa::Padding::PKCS1
		);

		println!("{:?}", result);

		match result
		{
			Err(_) => None,
			Ok(s) => Some(decrypted.to_vec()),
		}
	}
}
