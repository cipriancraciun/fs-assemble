

use crate::prelude::*;




pub type Outcome<Value> = Result<Value, io::Error>;




pub(crate) fn log (_slug : &str, _level : u16, _code : u32, _message : impl fmt::Display) -> () {
	if (_level != 0) && (_level < DUMP_LOG_LEVEL) {
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
	unsafe {
		_log_empty = false;
		_log_cut_last = false;
	}
}

pub(crate) fn log_cut (_forced : bool) -> () {
	if ! DUMP_LOG_CUT && ! _forced {
		return;
	}
	unsafe {
		if _log_cut_last {
			return;
		}
	}
	eprintln! ("[--]");
	unsafe {
		_log_empty = false;
		_log_cut_last = true;
	}
}

#[ allow (non_upper_case_globals) ]
static mut _log_empty : bool = true;
#[ allow (non_upper_case_globals) ]
static mut _log_cut_last : bool = false;




pub(crate) fn error (_code : u32, _message : impl fmt::Display) -> io::Error {
	
	let _message = format! ("[{:08x}]  {}", _code, _message);
	
	io::Error::new (io::ErrorKind::Other, _message)
}

