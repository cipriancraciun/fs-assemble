

use crate::prelude::*;




fn main_0 (_script : &Path, _source : &Path, _target : &Path) -> Result<(), io::Error> {
	
	let mut _filter = fsas::FilterRules::new ();
	
	let mut _index = Vec::with_capacity (16 * 1024);
	
	fsas::index (_source, &_filter, &mut _index) ?;
	
	fail! (0x84c61b84, "not implemented!");
}




pub fn main () -> ! {
	
	let _arguments = env::args_os () .collect::<Vec<_>> ();
	
	if _arguments.len () != 4 {
		log_error! (0xa09ad875, "invalid arguments count;  expected script, source and target!");
		process::exit (1);
	}
	
	match main_0 (_arguments[1].as_ref (), _arguments[2].as_ref (), _arguments[3].as_ref ()) {
		Ok (()) =>
			process::exit (0),
		Err (_error) => {
			log_error! (0x5c0e181c, "unexpected error encountered!  aborting!");
			log_error! (0, "{}", _error);
			process::exit (1);
		}
	}
}

