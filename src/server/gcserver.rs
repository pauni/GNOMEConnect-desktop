use serde_json;
use server::devicemanager;
use server::packets;

use std::io::{Read, Write, BufRead, BufReader,BufWriter};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{self, SyncSender, Receiver};
use std::thread;
use std::net::SocketAddr;




// all connections related stuff here
// device pairing, encryption, etc
// handly of userdata in seperate module


const PROTOCOL_VERSION: i64 = 45;




pub fn spawn_server(bind_addr: &'static str, queue_size: usize) -> Option<Receiver<StreamHandler>> {
	let (server_tx, server_rx) = mpsc::sync_channel(queue_size);

	thread::Builder::new()
		.name("GcEventServer".into())
		.spawn(move || {
			start_listener_loop(bind_addr, server_tx)
		});


	Some(server_rx)
}






pub fn start_listener_loop(bind_addr: &'static str, gcserver_channel: SyncSender<StreamHandler>) {
	info!("start server at {}", bind_addr);
	let tcp_server = match TcpListener::bind(bind_addr) {
		Ok(s) => s,
		Err(e) => panic!("can't bind to {}: {}", bind_addr, e),
	};


	debug!("start listening loop");

	for stream in tcp_server.incoming() {
		debug!("TCPconnection established");

		match stream {
			Ok(r) => gcserver_channel.send(StreamHandler::new(r).unwrap()),
			Err(e) => panic!("can't accexpt stream: {}", e),
		};

		debug!("waiting for connection...");
	};
}








#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub enum Type {
	PairRequest(packets::Pairing),
	EncryptedData(String),
}





#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Package {
	fingerprint: String,
	pub what: Type
}



/// Handle for the raw Stream
pub struct StreamHandler {
	stream: TcpStream,
	remote_ip: SocketAddr,
}



impl StreamHandler {
	pub fn new(stream: TcpStream) -> Option<Self> {
		warn!("Todo: devicemanager integration");

		let remote_addr = stream.peer_addr().unwrap();
		let mut buf_stream = BufReader::new(stream);

		// read the data
		let mut data = String::new();
		buf_stream.read_line(&mut data).unwrap();


		debug!("{}", data);
		debug!("read {} bytes from {}", data.len(), remote_addr);



		let package: Package = match serde_json::from_str(&data) {
			Ok(r) => {
				debug!("parsed packet successfully");
				r
			},
			Err(e) => {
				error!("jesus christ, it's not Json Bourne: {}", e);
				return None;
			},
		};


		let dm = devicemanager::DeviceManager::new();

		match package.what {
			Type::PairRequest(request) => {
				// handle the pairrequest here

				println!("{:#?}", request);
			},

			Type::EncryptedData(data) => {
				// decrypt data using the provided keys

				if !dm.is_paired(package.fingerprint) {
					buf_stream.into_inner().write_all();
				}



			}

		}


		Some(Self {
			stream: buf_stream.into_inner(),
			remote_ip: remote_addr,
		})
	}










	pub fn read_line(self) -> String {
		let mut buf_stream = BufReader::new(self.stream);

		// read the data
		let mut data = String::new();
		buf_stream.read_line(&mut data).unwrap();

		debug!("read {:6} bytes", data.len());

		// hand back the stream

		data
	}



	pub fn remote_ip(&self) -> SocketAddr {
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
	fn new() -> Self {
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


		Self {
			dev_mngr: devicemanager::DeviceManager::new()
		}
	}
}
