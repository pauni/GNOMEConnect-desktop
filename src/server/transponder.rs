use hostname;
use std::net::UdpSocket;
use std::thread;
use serde_json::to_string as to_json;
use crypto::md5;
use crypto::digest::Digest;



static BIND_ADDR: &str = "0.0.0.0:4112";
const BUFFER_SIZE: usize = 65536;






pub fn start(public_key: String) -> Option<()> {
	info!("start discovery service at {}", BIND_ADDR);

	match UdpSocket::bind(BIND_ADDR) {
		Ok(socket) => {
			thread::Builder::new()
				.name("transponder".into())
				.spawn(move || {
					transponder_loop(socket, public_key.clone())
				});

			Some(())
		},
		Err(e) => panic!("binding to {} failed: {}", BIND_ADDR, e)
	}
}




#[derive(Clone, Debug, Serialize)]
struct EchoSignal {
	device_name: String,
	fingerprint: String,
	os: String,
	public_key: String,
	checksum: String,
}







fn transponder_loop(udp_sock: UdpSocket, public_key: String) {
	loop {
		let mut buffer = [0; BUFFER_SIZE];
		let (length, mut remote_addr) = udp_sock.recv_from(&mut buffer).unwrap();


		let mut hasher = md5::Md5::new();
		hasher.input_str(&public_key);
		let checksum = hasher.result_str();


		// debug!("received discovery from {}", remote_addr);


		let echo = EchoSignal {
			device_name: hostname::get_hostname().unwrap(),
			fingerprint: "todo".into(),
			os: "debian".into(),
			public_key: public_key.clone(),
			checksum: checksum,
		};


		remote_addr.set_port(4112);

		let send = to_json(&echo).unwrap();
		udp_sock.send_to(&send.clone().into_bytes(), remote_addr).unwrap();

		debug!("responding {}", send);
	}
}
