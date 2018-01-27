use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;















#[derive(Debug, Clone)]
pub struct Rsa {
	priv_key: String,
	pub_key: String,
}


impl Rsa {
	pub fn generate(length: u32) -> Self
	{

		let private_key = gen_private_key(length);
		let public_key = gen_public_key(private_key.clone());


		println!("private: {}", private_key);
		println!("public: {}", public_key);



		Self {
			priv_key: private_key,
			pub_key: public_key,
		}
	}
}




fn gen_private_key(length: u32) -> String
{
	debug!("generate private key");
	let child = Command::new("/usr/bin/openssl")
		.arg("genrsa")
		.arg(&length.to_string())
		.stderr(Stdio::null())
		.stdout(Stdio::piped())
		.stdin(Stdio::null())
		.spawn()
		.expect("can't generate keys");


	let mut output = String::new();
	child.stdout.unwrap().read_to_string(&mut output);


	output
}



fn gen_public_key(private: String) -> String
{

	// openssl rsa -in priv.key -pubout
	let child = Command::new("/usr/bin/openssl")
		.arg("rsa")
		.arg("-pubout")
		.stderr(Stdio::null())
		.stdout(Stdio::piped())
		.stdin(Stdio::piped())
		.spawn()
		.expect("can't generate keys");


	let mut output = String::new();
	child.stdin.unwrap().write_all(private.as_bytes());

	child.stdout.unwrap().read_to_string(&mut output);

	output
}
