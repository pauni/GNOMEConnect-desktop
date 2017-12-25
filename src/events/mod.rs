// use serde_derive;
// use serde_json;
// use serde;



// TODO: move to dedicated mod

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "report_type", content = "report")]
pub enum Report {
	/// Battery status
	Power(Power),
	Notification(Notification),
}





#[derive(Debug, Serialize, Deserialize)]
#[serde()]
pub struct Power {
	pub level: u64,
	pub charging: bool
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
	pub icon: String,
	pub text: String,
	pub title: String,
	pub persistent: bool,
	pub program: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Connectivity {
	pub wifi_connected: bool,
	pub wifi_ssid: Option<String>,
	pub wifi_signal_strength: i64,
	pub local_ip: String,
	pub cell_network: String
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Misc {
	pub phone_unlocked: bool,
	pub display_active: bool,
}











//----------------------//
// Available  datatypes //
//----------------------//
