use base64;
use openssl::pkey;
use openssl::rsa;
use serde_json;
use server::devicemanager;
use server::packets;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ops::Add;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;
use serde::Serialize;




// all connections related stuff here
// device pairing, encryption, etc
// handly of userdata in seperate module


const PROTOCOL_VERSION: i64 = 45;
// TODO: PLACEHOLDER!!!!!!!!!
const PAIRINGMODE: bool = true;




pub struct GCServer {
	stream: TcpListener,
}




impl GCServer {
	pub fn spawn_server(bind_addr: &'static str) -> Option<Self>
	{
		info!("start server at {}", bind_addr);

		match TcpListener::bind(bind_addr)
		{
			Ok(s) => Some(Self { stream: s }),
			Err(e) => panic!("can't bind to {}: {}", bind_addr, e),
		}
	}
}


impl Iterator for GCServer {
	type Item = StreamHandler;

	fn next(&mut self) -> Option<Self::Item>
	{
		let (stream, addr) = self.stream.accept().unwrap();

		Some(StreamHandler::new(stream).unwrap())
	}
}










#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndroductionPackage {
	public_key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionMethod {
	Asym,
	Sym,
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Code {
	Ok,
	Unpaired,
	UnknownError,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPackage {
	code: Code,
	message: String,
}



/// Handle for the raw Stream and encryption
#[derive(Debug)]
pub struct StreamHandler {
	stream: TcpStream,
	remote_ip: SocketAddr,
	remote_public_key: rsa::Rsa<pkey::Public>,
	device_manager: devicemanager::DeviceManager,
	base64conf: base64::Config,
}


/// receives first informations and tells the client about us
impl StreamHandler {
	pub fn new(stream: TcpStream) -> Option<Self>
	{
		// TODO: devicemanager integration


		info!("incoming connection");

		let remote_addr = stream.peer_addr().unwrap();
		let mut buf_stream = BufReader::new(stream);

		// read the data
		let mut data = String::new();
		buf_stream.read_line(&mut data).unwrap();
		let mut stream = buf_stream.into_inner();



		debug!("read {} bytes from {}", data.len(), remote_addr);


		let package: IndroductionPackage = serde_json::from_str(&data).unwrap();



		let remote_key = rsa::Rsa::public_key_from_pem(
			package.public_key.as_bytes()
		).unwrap();

		let dm = devicemanager::DeviceManager::new();



		let response = IndroductionPackage {
			public_key: String::from_utf8(dm.get_public_key()).unwrap()
		};




		writeln!(stream, "{}", serde_json::to_string(&response).unwrap());

		// stream.write_all(serde_json::to_string(&response).unwrap().add("\n").as_bytes());

		let base64_opt = base64::Config::new(
			base64::CharacterSet::Standard,
			true,
			true,
			base64::LineWrap::NoWrap,
		);


		Some(
			Self {
				stream: stream,
				remote_ip: remote_addr,
				remote_public_key: remote_key,
				device_manager: dm,
				base64conf: base64_opt,
			}
		)
	}




	pub fn recv_package(&mut self) -> packets::TransportPackage
	{
		debug!("receive package");

		let line = self.read_line();

		println!("{}", line);


		let base64_opt = base64::Config::new(
			base64::CharacterSet::Standard,
			true,
			true,
			base64::LineWrap::NoWrap,
		);


		let encrypted = base64::decode_config(&line, base64_opt).expect("can't decode base64");

		let dm = &self.device_manager;
		let decrypted = dm.decrypt_asym(encrypted).unwrap();

		let line = String::from_utf8(decrypted).unwrap();

		serde_json::from_str(&line).unwrap()
	}



	pub fn send_package<T: Serialize>(&mut self, data: T)
	{
		debug!("send package");

		let json = serde_json::to_vec(&data).unwrap();
		let mut encrypted: Vec<u8> = Vec::new();


		self.remote_public_key.public_decrypt(
			&json,
			encrypted.as_mut_slice(),
			rsa::Padding::PKCS1_OAEP
		).unwrap();

		let base64 = base64::encode_config(&encrypted, self.base64conf);


		writeln!(self.stream, "{}", base64);
	}




	pub fn read_line(&mut self) -> String
	{
		let mut line = String::new();
		BufReader::new(&self.stream)
			.read_line(&mut line)
			.expect("can't read line");

		debug!("read {:6} bytes", line.len());
		line
	}



	pub fn remote_ip(&self) -> SocketAddr
	{
		self.remote_ip
	}
}
