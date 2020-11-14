

use crate::prelude::*;




fn main_0 (_script : &Path, _source_root : &Path, _target_root : &Path) -> Result<(), io::Error> {
	
	use crate::fsas::*;
	
	
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
	log_notice! (0x787ec493, "indexing source `{}`...", _source_root.display ());
	let mut _sources_existing = EntryVec::new ();
	let mut _sources_unhandled = EntryVec::new ();
	index (_source_root, &_source_filter, &mut _sources_existing) ?;
	log_cut! ();
	
	log_cut! ();
	log_notice! (0xcb4b5581, "indexing target `{}`...", _target_root.display ());
	let mut _targets_existing = EntryVec::new ();
	let mut _targets_unhandled = EntryVec::new ();
	index (_target_root, &_target_filter, &mut _targets_existing) ?;
	log_cut! ();
	
	let mut _descriptors_planned = TargetDescriptorVec::new ();
	let mut _descriptors_skipped = TargetDescriptorVec::new ();
	
	log_cut! ();
	log_notice! (0x7827e63b, "planning...");
	let _plan = plan (
			&_target_rules,
			_source_root,
			_sources_existing,
			&mut _sources_unhandled,
			_target_root,
			_targets_existing,
			&mut _targets_unhandled,
			&mut _descriptors_planned,
			&mut _descriptors_skipped,
		) ?;
	log_cut! ();
	
	if true {
		trace_entries (_sources_unhandled.iter (), Some ("sources unhandled:"));
		trace_entries (_targets_unhandled.iter (), Some ("targets unhandled:"));
	}
	
	log_cut! ();
	log_notice! (0x01f9fc36, "verifying...");
	verify (&_descriptors_planned) ?;
	log_cut! ();
	
	let mut _descriptors_required = TargetDescriptorVec::new ();
	let mut _descriptors_succeeded = TargetDescriptorVec::new ();
	let mut _descriptors_failed = TargetDescriptorVec::new ();
	
	log_cut! ();
	log_notice! (0x6982871d, "simplifying...");
	simplify (
			_source_root,
			_target_root,
			_descriptors_planned,
			&mut _descriptors_required,
			&mut _descriptors_skipped,
		) ?;
	log_cut! ();
	
	if true {
		fsas::trace_descriptors (_descriptors_required.iter (), Some ("descriptors required:"));
	}
	
	log_cut! ();
	log_notice! (0xc38cec3a, "executing...");
	execute (
			_source_root,
			_target_root,
			_descriptors_required,
			&mut _descriptors_succeeded,
			&mut _descriptors_failed,
			&mut _descriptors_skipped,
		) ?;
	log_cut! ();
	
	if true {
		trace_descriptors (_descriptors_failed.iter (), Some ("descriptors failed:"));
		trace_descriptors (_descriptors_succeeded.iter (), Some ("descriptors succeeded:"));
		trace_descriptors (_descriptors_skipped.iter (), Some ("descriptors skipped:"));
	}
	
	return Ok (());
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

