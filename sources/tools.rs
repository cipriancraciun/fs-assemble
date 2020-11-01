

use crate::prelude::*;




pub type Outcome<Value> = Result<Value, io::Error>;




pub fn log (_slug : &str, _level : u16, _code : u32, _message : impl fmt::Display) -> () {
	if _level < 20000 {
		return;
	}
	match (_slug, _code) {
		("", 0) =>
			eprintln! ("{}", _message),
		("", _) =>
			eprintln! ("[{:08x}]  {}", _code, _message),
		(_, 0) =>
			eprintln! ("{} {}", _slug, _message),
		(_, _) =>
			eprintln! ("{} [{:08x}]  {}", _slug, _code, _message),
	}
}


pub fn error (_code : u32, _message : impl fmt::Display) -> io::Error {
	
	let _message = format! ("[{:08x}]  {}", _code, _message);
	
	io::Error::new (io::ErrorKind::Other, _message)
}

