

use crate::prelude::*;
use crate::rules::*;




pub fn plan (_rules : &TargetRules, _sources : Vec<Entry>, _targets : Vec<Entry>) -> Outcome<Vec<TargetEntry>> {
	
	
	// ----
	
	
	let mut _sources_existing = fsas::build_tree (_sources) ?;
	let mut _sources_unhandled = BTreeMap::new ();
	
	let mut _targets_existing = fsas::build_tree (_targets) ?;
	let mut _targets_planned = Vec::new ();
	let mut _targets_unhandled = Vec::new ();
	
	
	// ----
	
	
	{
		log_debug! (0x9f16c940, "sifting sources...");
		
		for _entry in _sources_existing.values () {
			let mut _handled = false;
			for _rule in _rules.rules.iter () {
				match _rule {
					
					
					TargetRule::Copy { source : _selector, target : _target } =>
						if _selector.matches (&_entry) ? {
							let _entry = TargetEntry {
									path : _target.clone (),
									operation : TargetOperation::Copy { existing : _entry.clone () },
								};
							_targets_planned.push (_entry);
							_handled = true;
						}
					
					TargetRule::CopyFlatten { source : _selector, target : _target } =>
						if _selector.matches (&_entry) ? {
							let _target = Path::new (_target) .join (&_entry.name) .into ();
							let _entry = TargetEntry {
									path : _target,
									operation : TargetOperation::Copy { existing : _entry.clone () },
								};
							_targets_planned.push (_entry);
							_handled = true;
						}
					
					TargetRule::CopyRename { .. } =>
						fail! (0xb2bb5d6d, "not implemented!"),
					
					
					TargetRule::Symlink { source : _selector, target : _target } =>
						if _selector.matches (&_entry) ? {
							let _entry = TargetEntry {
									path : _target.clone (),
									operation : TargetOperation::Symlink { existing : _entry.clone () },
								};
							_targets_planned.push (_entry);
							_handled = true;
						}
					
					TargetRule::SymlinkFlatten { source : _selector, target : _target } =>
						if _selector.matches (&_entry) ? {
							let _target = Path::new (_target) .join (&_entry.name) .into ();
							let _entry = TargetEntry {
									path : _target,
									operation : TargetOperation::Symlink { existing : _entry.clone () },
								};
							_targets_planned.push (_entry);
							_handled = true;
						}
					
					TargetRule::SymlinkRename { .. } =>
						fail! (0x3d416349, "not implemented!"),
					
					
					TargetRule::Protect { .. } |
					TargetRule::Clean { .. } =>
						(),
					
					TargetRule::MakeDir { .. } |
					TargetRule::MakeSymlink { .. } =>
						(),
				}
			}
			
			if ! _handled {
				_sources_unhandled.insert (_entry.path.clone (), _entry.clone ());
			}
		}
	}
	
	
	// ----
	
	
	{
		log_debug! (0x1f72d23e, "sifting targets...");
		
		for _entry in _targets_existing.values () {
			let mut _handled = false;
			for _rule in _rules.rules.iter () {
				match _rule {
					
					
					TargetRule::Protect { target : _selector } =>
						if _selector.matches (&_entry) ? {
							let _entry = TargetEntry {
									path : _entry.path.clone (),
									operation : TargetOperation::Protect { existing : _entry.clone () },
								};
							_targets_planned.push (_entry);
							_handled = true;
							break;
						}
					
					TargetRule::Clean { target : _selector } =>
						if _selector.matches (&_entry) ? {
							let _entry = TargetEntry {
									path : _entry.path.clone (),
									operation : TargetOperation::Unlink,
								};
							_targets_planned.push (_entry);
							_handled = true;
							break;
						}
					
					
					TargetRule::Copy { .. } |
					TargetRule::CopyFlatten { .. } |
					TargetRule::CopyRename { .. } =>
						(),
					
					TargetRule::Symlink { .. } |
					TargetRule::SymlinkFlatten { .. } |
					TargetRule::SymlinkRename { .. } =>
						(),
					
					TargetRule::MakeDir { .. } |
					TargetRule::MakeSymlink { .. } =>
						(),
				}
			}
			
			if ! _handled {
				_targets_unhandled.push (_entry.clone ());
			}
		}
	}
	
	
	// ----
	
	
	{
		log_debug! (0xcc7d8038, "sifting directives...");
		
		for _rule in _rules.rules.iter () {
			match _rule {
				
				TargetRule::MakeDir { target : _target } => {
					let _entry = TargetEntry {
							path : _target.clone (),
							operation : TargetOperation::MakeDir,
						};
					_targets_planned.push (_entry);
				}
				
				TargetRule::MakeSymlink { target : _target, link : _link } => {
					let _entry = TargetEntry {
							path : _target.clone (),
							operation : TargetOperation::MakeSymlink { link : _link.clone () },
						};
					_targets_planned.push (_entry);
				}
				
				
				TargetRule::Protect { .. } |
				TargetRule::Clean { .. } =>
					(),
				
				TargetRule::Copy { .. } |
				TargetRule::CopyFlatten { .. } |
				TargetRule::CopyRename { .. } =>
					(),
				
				TargetRule::Symlink { .. } |
				TargetRule::SymlinkFlatten { .. } |
				TargetRule::SymlinkRename { .. } =>
					(),
			}
		}
	}
	
	
	// ----
	
	
	{
		log_debug! (0x62d40e83, "extending copy...");
		
		let mut _targets_planned_extended = Vec::new ();
		
		for _target_0 in _targets_planned.iter () {
			match &_target_0.operation {
				
				TargetOperation::Copy { existing : _source_0 } =>
					if _source_0.is_dir {
						for (_, _source_1) in _sources_existing.range::<OsString, _> ((Bound::Excluded (&_source_0.path), Bound::Unbounded)) {
							if ! Path::new (&_source_1.path) .starts_with (&_source_0.path) {
								break;
							}
							let _target_1_path = Path::new (&_target_0.path) .join (Path::new (&_source_1.path) .strip_prefix (&_source_0.path) .unwrap ()) .into ();
							let _target_1 = TargetEntry {
									path : _target_1_path,
									operation : TargetOperation::Copy { existing : _source_1.clone () },
								};
							_targets_planned_extended.push (_target_1);
						}
					},
				
				_ =>
					(),
			}
		}
		
		_targets_planned.append (&mut _targets_planned_extended);
	}
	
	
	// ----
	
	
	{
		log_debug! (0x1e7e28ce, "extending mkdir...");
		
		let mut _target_mkdirs = HashSet::new ();
		
		for _target in _targets_planned.iter () {
			for _parent in Path::new (&_target.path) .ancestors () {
				let _parent = _parent.as_os_str ();
				match _parent {
					_parent if _parent == _target.path =>
						(),
					_parent if (_parent == ".") || (_parent == "..") || (_parent == "") =>
						fail! (0xab2bbf7b, "invalid state!"),
					_parent => {
						_target_mkdirs.insert (OsString::from (_parent));
					}
				}
			}
		}
		
		for _target in _target_mkdirs.into_iter () {
			let _entry = TargetEntry {
					path : _target,
					operation : TargetOperation::MakeDir,
				};
			_targets_planned.push (_entry);
		}
	}
	
	
	// ----
	
	
	_targets_planned.sort_by (|_left, _right|  OsStr::cmp (&_left.path, &_right.path));
	
	for _target in _targets_planned.iter () {
		log_debug! (0x096428c7, "{:?}", _target);
	}
	
	
	// ----
	
	fail! (0x1d81ea47, "not implemented");
}

