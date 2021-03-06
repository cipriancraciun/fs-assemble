

use crate::prelude::*;
use crate::lib::*;




pub fn main_0 (_script_path : &Path, _source_root : &Path, _target_root : &Path) -> Result<(), io::Error> {
	
	
	// ----
	
	
	log_cut! ();
	log_information! (0x96598390, "loading script `{}`...", _script_path.display ());
	let _script = parse (_script_path) ?;
	log_cut! ();
	
	if DUMP_SCRIPT {
		trace_script (&_script, Some ("script:"));
	}
	
	
	// ----
	
	
	let mut _source_filter = IndexRules::new_for_source ();
	let mut _target_filter = IndexRules::new_for_target ();
	let mut _target_rules = TargetRules::new ();
	
	for _statement in _script.statements.iter () {
		match _statement {
			
			Statement::SourceIndexOptions (_options) =>
				for _option in _options.iter () .cloned () {
					_source_filter.push_option (_option);
				}
			Statement::TargetIndexOptions (_options) =>
				for _option in _options.iter () .cloned () {
					_target_filter.push_option (_option);
				}
			
			Statement::SourceIndexRules (_rules) =>
				_source_filter.rules.extend (_rules.iter () .cloned ()),
			Statement::TargetIndexRules (_rules) =>
				_source_filter.rules.extend (_rules.iter () .cloned ()),
			
			Statement::TargetRule (_rule) =>
				_target_rules.rules.push (_rule.clone ()),
			Statement::TargetRules (_rules) =>
				_target_rules.rules.extend (_rules.iter () .cloned ()),
			
			Statement::Commented =>
				(),
		}
	}
	
	
	// ----
	
	
	log_cut! ();
	log_information! (0x787ec493, "indexing source `{}`...", _source_root.display ());
	let mut _sources_existing = EntryVec::new ();
	let mut _sources_unhandled = EntryVec::new ();
	index (_source_root, &_source_filter, &mut _sources_existing) ?;
	log_cut! ();
	
	log_cut! ();
	log_information! (0xcb4b5581, "indexing target `{}`...", _target_root.display ());
	let mut _targets_existing = EntryVec::new ();
	let mut _targets_unhandled = EntryVec::new ();
	index (_target_root, &_target_filter, &mut _targets_existing) ?;
	log_cut! ();
	
	
	// ----
	
	
	let mut _descriptors_planned = TargetDescriptorVec::new ();
	let mut _descriptors_skipped = TargetDescriptorVec::new ();
	
	log_cut! ();
	log_information! (0x7827e63b, "planning...");
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
	
	if DUMP_SOURCES_UNHANDLED {
		trace_entries (_sources_unhandled.iter (), Some ("sources unhandled:"));
	}
	if DUMP_TARGETS_UNHANDLED {
		trace_entries (_targets_unhandled.iter (), Some ("targets unhandled:"));
	}
	
	if DUMP_DESCRIPTORS_PLANNED {
		trace_descriptors (_descriptors_planned.iter (), Some ("descriptors planned:"));
	}
	
	log_cut! ();
	log_information! (0x01f9fc36, "verifying (1)...");
	verify (&_descriptors_planned) ?;
	log_cut! ();
	
	
	// ----
	
	
	let mut _descriptors_required = TargetDescriptorVec::new ();
	let mut _descriptors_succeeded = TargetDescriptorVec::new ();
	let mut _descriptors_failed = TargetDescriptorVec::new ();
	
	log_cut! ();
	log_information! (0x6982871d, "simplifying...");
	simplify (
			_source_root,
			_target_root,
			_descriptors_planned,
			&mut _descriptors_required,
			&mut _descriptors_skipped,
		) ?;
	log_cut! ();
	
	if DUMP_DESCRIPTORS_REQUIRED {
		trace_descriptors (
				_descriptors_required.iter () .filter (|_descriptor| if let TargetOperation::Protect = _descriptor.operation { false } else { true }),
				Some ("descriptors required:"));
	}
	
	log_cut! ();
	log_information! (0x4dfe419e, "verifying (2)...");
	verify (&_descriptors_required) ?;
	log_cut! ();
	
	
	// ----
	
	
	log_cut! ();
	log_information! (0xc38cec3a, "executing...");
	execute (
			_source_root,
			_target_root,
			_descriptors_required,
			&mut _descriptors_succeeded,
			&mut _descriptors_failed,
			&mut _descriptors_skipped,
		) ?;
	log_cut! ();
	
	if DUMP_DESCRIPTORS_FAILED {
		trace_descriptors (_descriptors_failed.iter (), Some ("descriptors failed:"));
	}
	if DUMP_DESCRIPTORS_SUCCEEDED {
		trace_descriptors (_descriptors_succeeded.iter (), Some ("descriptors succeeded:"));
	}
	if DUMP_DESCRIPTORS_SKIPPED {
		trace_descriptors (_descriptors_skipped.iter (), Some ("descriptors skipped:"));
	}
	
	
	// ----
	
	
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

