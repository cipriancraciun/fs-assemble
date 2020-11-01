

use ::std::fmt;
use ::std::io;




pub fn log (_slug : &str, _code : u32, _message : impl fmt::Display) -> () {
	match (_slug, _code) {
		("", 0) =>
			eprintln! ("{}", _message),
		("", _) =>
			eprintln! ("[{:8x}]  {}", _code, _message),
		(_, 0) =>
			eprintln! ("{} {}", _slug, _message),
		(_, _) =>
			eprintln! ("{} [{:8x}]  {}", _slug, _code, _message),
	}
}


pub fn error (_code : u32, _message : impl fmt::Display) -> io::Error {
	
	let _message = format! ("[{:8x}]  {}", _code, _message);
	
	io::Error::new (io::ErrorKind::Other, _message)
}

