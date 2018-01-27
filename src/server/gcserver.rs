use serde_json;
use server::devicemanager;
use server::packets;

use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;
use std::ops::Add;
use base64;
use openssl::rsa;




// all connections related stuff here
// device pairing, encryption, etc
// handly of userdata in seperate module


const PROTOCOL_VERSION: i64 = 45;
// TODO: PLACEHOLDER!!!!!!!!!
const PAIRINGMODE: bool = true;



pub fn spawn_server(bind_addr: &'static str, queue_size: usize) -> Option<Receiver<StreamHandler>>
{
	let (server_tx, server_rx) = mpsc::sync_channel(queue_size);

	thread::Builder::new()
		.name("GcEventServer".into())
		.spawn(move || start_listener_loop(bind_addr, server_tx));


	Some(server_rx)
}






pub fn start_listener_loop(bind_addr: &'static str, gcserver_channel: SyncSender<StreamHandler>)
{
	info!("start server at {}", bind_addr);
	let tcp_server = match TcpListener::bind(bind_addr)
	{
		Ok(s) => s,
		Err(e) => panic!("can't bind to {}: {}", bind_addr, e),
	};


	debug!("start listening loop");

	for stream in tcp_server.incoming() {
		debug!("TCPconnection established");

		match stream
		{
			Ok(r) => gcserver_channel.send(StreamHandler::new(r).unwrap()),
			Err(e) => panic!("can't accexpt stream: {}", e),
		};

		debug!("waiting for connection...");
	}
}













#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndroductionPackage {
	public_key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionMethod {
	Asym,
	Sym
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Code {
	Ok,
	Unpaired,
	UnknownError
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
	remote_public_key: Vec<u8>,
	device_manager: devicemanager::DeviceManager,
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



		debug!("{}", data);
		debug!("read {} bytes from {}", data.len(), remote_addr);


		println!("{}", data);

		let package: IndroductionPackage = serde_json::from_str(&data).unwrap();


		println!("{:#?}", package);

		let remote_key = package.public_key;
		let dm = devicemanager::DeviceManager::new();



		let response = IndroductionPackage {
			public_key: String::from_utf8(dm.get_public_key()).unwrap()
		};




		writeln!(stream, "{}", serde_json::to_string(&response).unwrap());

		// stream.write_all(serde_json::to_string(&response).unwrap().add("\n").as_bytes());



		Some(
			Self {
				stream: stream,
				remote_ip: remote_addr,
				remote_public_key: remote_key.as_bytes().to_vec(),
				device_manager: dm,
			}
		)
	}




	pub fn recv_package(self) -> packets::TransportPackage {
		let mut bufreader = BufReader::new(self.stream);

		let mut line = String::new();
		bufreader.read_line(&mut line);

		println!("{:?}", line);


		let encrypted = base64::decode(&line).unwrap();

		let dm = self.device_manager;
		let decrypted = dm.decrypt_asym(encrypted).unwrap();

		let line = String::from_utf8(decrypted).unwrap();

		serde_json::from_str(&line).unwrap()
	}




	pub fn read_line(self) -> String
	{
		let mut buf_stream = BufReader::new(self.stream);

		// read the data
		let mut data = String::new();
		buf_stream.read_line(&mut data).unwrap();

		debug!("read {:6} bytes", data.len());

		// hand back the stream

		data
	}



	pub fn remote_ip(&self) -> SocketAddr
	{
		self.remote_ip
	}
}



// impl Drop for StreamHandler {
// 	fn drop(&mut self) {
// 		debug!("dropped StreamHandler");
// 	}
// }













pub struct GCServer {
	dev_mngr: devicemanager::DeviceManager,
}





impl GCServer {
	fn new() -> Self
	{
		// let tcp_server = match TcpListener::bind(super::BIND_ADDR) {
		//     Ok(s) => s,
		//     Err(e) => panic!("can't bind to {}: {}", super::BIND_ADDR, e),
		// };
		//
		//
		// thread::spawn(move || {
		//     info!("start listening at {}", super::BIND_ADDR);
		//     start_listener_loop(tcp_server);
		// }).join();


		Self { dev_mngr: devicemanager::DeviceManager::new() }
	}
}
