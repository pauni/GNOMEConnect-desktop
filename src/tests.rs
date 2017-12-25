extern crate serde_derive;
// extern crate serde;
extern crate serde_json;


pub mod events;









#[cfg(test)]
pub mod tests {
	use events;
	use serde_json;
	#[test]
	pub fn json_decoding() {
		let json_raw = r#"
			[
				{
					"meta": "BatteryStatus",
					"level": 86,
					"charging": false
				},
				{
					"meta": "Notification",
					"title": "Test Notification",
					"text": "foobar",
					"icon": "Base64EncodedIcon"
				}
			]
		"#;


		let data: Vec<events::Event> = serde_json::from_str(json_raw).unwrap();

		match data[0] {
			events::Event::BatteryStatus{level, charging} => {
				assert_eq!(level, 86);
				assert_eq!(charging, false);
			},
			_ => panic!("whut?")
		}


		eprintln!("{:#?}", data);
	}
}
