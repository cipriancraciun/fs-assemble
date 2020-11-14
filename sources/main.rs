

use crate::prelude::*;




fn main_0 (_script : &Path, _source_root : &Path, _target_root : &Path) -> Result<(), io::Error> {
	
	use fsas::*;
	
	
	let mut _source_filter = IndexRules::new_for_source ();
	_source_filter
			.push_exclude (EntrySelector::if_matches_path (Pattern::exact ("/.git")))
			.push_exclude (EntrySelector::if_matches_path (Pattern::exact ("/target")));
	
	let mut _target_filter = IndexRules::new_for_target ();
	
	
	let mut _target_rules = TargetRules::new ();
	_target_rules
			.push_copy ("/Cargo.toml", "/Cargo.toml")
			.push_symlink ("/Cargo.lock", "/Cargo.lock")
			.push_copy ("/sources", "/sources")
			.push_make_dir ("/1/2/3/4")
			.push_protect ("/target/**")
			.push_make_symlink ("/target/null", "/dev/null")
			.push_unlink ("/**");
	
	
	log_cut! ();
	log_notice! (0x787ec493, "indexing source path `{}`...", _source_root.display ());
	let mut _source_entries = Vec::with_capacity (16 * 1024);
	index (_source_root, &_source_filter, &mut _source_entries) ?;
	log_cut! ();
	
	log_cut! ();
	log_notice! (0xcb4b5581, "indexing target path `{}`...", _target_root.display ());
	let mut _target_entries = Vec::with_capacity (16 * 1024);
	index (_target_root, &_target_filter, &mut _target_entries) ?;
	log_cut! ();
	
	log_cut! ();
	log_notice! (0x7827e63b, "planning...");
	let _target_entries = plan (&_target_rules, _source_root, _source_entries, _target_root, _target_entries) ?;
	log_cut! ();
	
	log_cut! ();
	for _target_entry in _target_entries.iter () {
		log_debug! (0xb7c38713, "{:?}", _target_entry);
	}
	log_cut! ();
	
	fail_unimplemented! (0x84c61b84);
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

