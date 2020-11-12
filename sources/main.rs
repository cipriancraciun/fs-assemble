

use crate::prelude::*;




fn main_0 (_script : &Path, _source_path : &Path, _target_path : &Path) -> Result<(), io::Error> {
	
	let mut _source_filter = fsas::IndexRules::new ();
	_source_filter.push_exclude (fsas::EntrySelector::if_matches_name (fsas::Pattern::exact ("target")));
	
	let mut _target_filter = fsas::IndexRules::new ();
	
	let mut _source_entries = Vec::with_capacity (16 * 1024);
	let mut _target_entries = Vec::with_capacity (16 * 1024);
	
	log_cut! ();
	log_notice! (0x787ec493, "indexing source path `{}`...", _source_path.display ());
	fsas::index (_source_path, &_source_filter, &mut _source_entries) ?;
	_source_entries.sort_by (|_left, _right| OsString::cmp (&_left.path, &_right.path));
	log_cut! ();
	
	log_cut! ();
	log_notice! (0xcb4b5581, "indexing target path `{}`...", _target_path.display ());
	fsas::index (_target_path, &_target_filter, &mut _target_entries) ?;
	_target_entries.sort_by (|_left, _right| OsString::cmp (&_left.path, &_right.path));
	log_cut! ();
	
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

