extern crate mysql;

use std::default::Default;
use mysql::conn::*;
//use mysql::conn::MyConn;
//use mysql::conn::MyOpts;

pub struct DbWriter {
	pub conn : MyConn
}

impl DbWriter {
	pub fn new() -> DbWriter  {
		let opts = MyOpts {
			tcp_addr: Some("52.68.112.89".to_string()),
			user: Some("root".to_string()),
			pass: Some("1q2w3e4r".to_string()),
			..Default::default()
		};
		
		//how can we control failed?
		let mut connection = MyConn::new(opts).unwrap();
		
		DbWriter { conn: connection }
	}
	
	//pub fn db_init() -> Result<bool, &'static str> {
	//}
}

#[test]
fn it_works() {
}
