use gnomeconnect::events;
use gnomeconnect::events::Report;
use hostname::get_hostname;
use packets::TransportHeader;
use serde_json;
use server::devicemanager;
use server::packets;
use server::packets::request;
use server::packets::response;
use std;
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






pub fn start_listener_loop(bind_addr: &'static str, gcserver_channel: SyncSender<StreamHandler>)  {

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



/// Handle for the raw Stream
pub struct StreamHandler {
	stream: TcpStream,
	remote_ip: SocketAddr,
	header: TransportHeader,
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


		let header: TransportHeader = match serde_json::from_str(&data) {
			Ok(r) => {
				debug!("parsed packet successfully");
				r
			},
			Err(e) => {
				error!("jesus christ, it's not Json Bourne: {}", e);
				return None;
			},
		};




		Some(Self {
			stream: buf_stream.into_inner(),
			remote_ip: remote_addr,
			header: header,
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


	pub fn write_line(&mut self, data: String) {
		debug!("write {} bytes", data.len());
		self.stream.write_all((data + "\n").as_bytes());
	}



	pub fn action(&self) -> packets::Action {
		self.header.type_.clone()
	}

	pub fn remote_ip(&self) -> SocketAddr {
		self.remote_ip
	}

	pub fn remote_id(&self) -> String {
		self.header.fingerprint.clone()
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
