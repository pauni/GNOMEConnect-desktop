pub mod gcserver;
pub mod transponder;
pub mod devicemanager;
pub mod packets;

use serde_json;
use events;


pub const BIND_ADDR: &str = "0.0.0.0:4112";
