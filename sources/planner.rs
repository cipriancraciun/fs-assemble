

use crate::prelude::*;
use crate::rules::*;




pub fn plan (
			_rules : &TargetRules,
			_sources_root : &Path,
			_sources_existing : EntryVec,
			_sources_unhandled : &mut EntryVec,
			_targets_root : &Path,
			_targets_existing : EntryVec,
			_targets_unhandled : &mut EntryVec,
			_descriptors_planned : &mut TargetDescriptorVec,
			_descriptors_skipped : &mut TargetDescriptorVec,
		) -> Outcome<()> {
	
	let mut _sources_existing = fsas::build_tree (_sources_existing) ?;
	let mut _sources_handled = PathSet::new ();
	
	let mut _targets_existing = fsas::build_tree (_targets_existing) ?;
	let mut _targets_handled = PathSet::new ();
	
	let mut _targets_pending = TargetDescriptorVec::new ();
	sift_sources (_rules, &_sources_existing, &mut _sources_handled, &_targets_existing, &mut _targets_pending) ?;
	sift_targets (_rules, &_targets_existing, &mut _targets_handled, &mut _targets_pending) ?;
	sift_directives (_rules, &_targets_existing, &mut _targets_pending) ?;
	
	let mut _targets_extended = TargetDescriptorVec::new ();
	extend_mkdir (&_targets_existing, &_targets_pending, &mut _targets_extended) ?;
	extend_copy (&_sources_existing, &mut _sources_handled, &_targets_existing, &_targets_pending, &mut _targets_extended) ?;
	
	let mut _targets_protect = TargetDescriptorMap::new ();
	let mut _targets_unlink_0 = TargetDescriptorMap::new ();
	let mut _targets_create_0 = TargetDescriptorMap::new ();
	sort_targets (_targets_extended, &mut _targets_protect, &mut _targets_unlink_0, &mut _targets_create_0) ?;
	
	let mut _targets_skipped = TargetDescriptorVec::new ();
	
	let mut _targets_unlink = TargetDescriptorMap::new ();
	prune_unlink (_targets_unlink_0, &_targets_create_0, &mut _targets_protect, &mut _targets_unlink, &mut _targets_skipped) ?;
	
	let mut _targets_create = TargetDescriptorMap::new ();
	prune_create (_sources_root, _targets_root, _targets_create_0, &mut _targets_protect, &mut _targets_create, &mut _targets_skipped) ?;
	
	if false {
		fsas::trace_descriptors (_targets_create.values (), Some ("descriptors planned for creation:"));
		fsas::trace_descriptors (_targets_protect.values (), Some ("descriptors planned for protection:"));
		fsas::trace_descriptors (_targets_unlink.values (), Some ("descriptors planned for unlinking:"));
		fsas::trace_descriptors (_targets_skipped.iter (), Some ("descriptors skipped:"));
	}
	
	for (_, _source) in _sources_existing.into_iter () {
		if ! _sources_handled.contains (&_source.path) {
			_sources_unhandled.push (_source);
		}
	}
	for (_, _target) in _targets_existing.into_iter () {
		if ! _targets_handled.contains (&_target.path) {
			_targets_unhandled.push (_target);
		}
	}
	
	_descriptors_planned.extend (_targets_protect.into_iter () .map (|(_, _descriptor)| _descriptor));
	_descriptors_planned.extend (_targets_unlink.into_iter () .map (|(_, _descriptor)| _descriptor) .rev ());
	_descriptors_planned.extend (_targets_create.into_iter () .map (|(_, _descriptor)| _descriptor));
	
	_descriptors_skipped.extend (_targets_skipped.into_iter ());
	
	return Ok (());
}




pub fn verify (_targets_planned : &TargetDescriptorVec) -> Outcome<()> {
	
	let mut _targets_protect = PathSet::new ();
	let mut _targets_unlink = PathSet::new ();
	let mut _targets_create = PathSet::new ();
	let mut _targets_dir = PathSet::new ();
	
	let mut _valid = true;
	let mut _seen_protect = false;
	let mut _seen_unlink = false;
	let mut _seen_create = false;
	
	for _descriptor in _targets_planned.iter () {
		
		let _path = &_descriptor.path;
		let _path_parent = Path::new (_path) .parent () .map (Path::as_os_str);
		let _path_display = Path::new (_path) .display ();
		
		// ----
		
		match &_descriptor.operation {
			
			TargetOperation::Protect => {
				if _seen_unlink {
					log_error! (0xd2281c1c, "invalid plan for `{}`:  protect after unlink!", _path_display);
					_valid = false;
				}
				if _seen_create {
					log_error! (0x759a33b1, "invalid plan for `{}`:  protect after create!", _path_display);
					_valid = false;
				}
				if _targets_unlink.contains (_path) {
					log_error! (0x8920569c, "invalid plan for `{}`:  protect and unlink!", _path_display);
					_valid = false;
				}
				if ! _targets_protect.insert (_path.clone ()) {
					log_error! (0xcad46f98, "invalid plan for `{}`:  duplicate protect!", _path_display);
					_valid = false;
				}
				if let Some (_target) = &_descriptor.existing {
					if _target.is_dir && ! _target.is_symlink {
						if ! _targets_dir.insert (_path.clone ()) {
							log_error! (0xb9b6b547, "invalid plan for `{}`:  duplicate folder!", _path_display);
							_valid = false;
						}
					}
				} else {
					log_error! (0x4e043540, "invalid plan for `{}`:  protect does not exist!", _path_display);
					_valid = false;
				}
				_seen_protect = true;
				continue;
			}
			
			TargetOperation::Unlink => {
				if _seen_create {
					log_error! (0x347c2dc2, "invalid plan for `{}`:  unlink after create!", _path_display);
					_valid = false;
				}
				if _targets_protect.contains (_path) {
					log_error! (0x50da48f3, "invalid plan for `{}`:  protect and unlink!", _path_display);
					_valid = false;
				}
				if ! _targets_unlink.insert (_path.clone ()) {
					log_error! (0x204b283f, "invalid plan for `{}`:  duplicate unlink!", _path_display);
					_valid = false;
				}
				if let Some (_path_parent) = _path_parent {
					if _targets_unlink.contains (_path_parent) {
						log_error! (0x7b26e762, "invalid plan for `{}`:  unlink after parent unlink!", _path_display);
						_valid = false;
					}
				}
				if let Some (_target) = &_descriptor.existing {
					// NOP
				} else {
					log_error! (0x12cfd6fb, "invalid plan for `{}`:  unlink does not exist!", _path_display);
					_valid = false;
				}
				_seen_unlink = true;
				continue;
			}
			
			_ => (),
		}
		
		// ----
		
		let mut _should_include_unlink = false;
		let mut _should_exclude_unlink = false;
		
		match &_descriptor.operation {
			
			TargetOperation::Protect =>
				unreachable! (),
			TargetOperation::Unlink =>
				unreachable! (),
			
			TargetOperation::Copy { source : _source } => {
				if ! _source.is_dir {
					if let Some (_target) = &_descriptor.existing {
						if _target.is_dir && ! _target.is_symlink {
							_should_include_unlink = true;
						} else {
							_should_exclude_unlink = true;
						}
					} else {
						_should_exclude_unlink = true;
					}
				} else {
					log_error! (0xf2c7f289, "invalid plan for `{}`:  unexpected copy!", _path_display);
					_valid = false;
				}
			}
			
			TargetOperation::Symlink { .. } => {
				log_error! (0x18ab6419, "invalid plan for `{}`:  unexpected symlink!", _path_display);
				_valid = false;
			}
			
			TargetOperation::MakeDir => {
				if let Some (_target) = &_descriptor.existing {
					if _target.is_dir && ! _target.is_symlink {
						_should_exclude_unlink = true;
					} else {
						_should_include_unlink = true;
					}
				} else {
					_should_exclude_unlink = true;
				}
				if ! _targets_dir.insert (_path.clone ()) {
					log_error! (0x0f5d2be3, "invalid plan for `{}`:  duplicate folder!", _path_display);
					_valid = false;
				}
			}
			
			TargetOperation::MakeSymlink { .. } => {
				if let Some (_target) = &_descriptor.existing {
					if _target.is_dir && ! _target.is_symlink {
						_should_include_unlink = true;
					} else {
						_should_exclude_unlink = true;
					}
				} else {
					_should_exclude_unlink = true;
				}
			}
		}
		
		
		if _targets_protect.contains (_path) {
			log_error! (0x260dd701, "invalid plan for `{}`:  protect and create!", _path_display);
			_valid = false;
		}
		if ! _targets_create.insert (_path.clone ()) {
			log_error! (0xbda03b7c, "invalid plan for `{}`:  duplicate create!", _path_display);
			_valid = false;
		}
		if let Some (_path_parent) = _path_parent {
			if ! _targets_dir.contains (_path_parent) {
				log_error! (0xbf2b9fe8, "invalid plan for `{}`:  parent missing!", _path_display);
				_valid = false;
			}
		}
		if _should_include_unlink & ! _targets_unlink.contains (_path) {
			log_error! (0x9dc8aaa8, "invalid plan for `{}`:  unlink missing!", _path_display);
			_valid = false;
		}
		if _should_exclude_unlink && _targets_unlink.contains (_path) {
			log_error! (0x250a90d0, "invalid plan for `{}`:  unlink superfluous!", _path_display);
			_valid = false;
		}
		_seen_create = true;
	}
	
	
	for _descriptors in _targets_planned.windows (2) {
		let _left = &_descriptors[0];
		let _right = &_descriptors[1];
		
		let _ordering = match (&_left.operation, &_right.operation) {
			
			(TargetOperation::Protect, TargetOperation::Protect) =>
				Some (Ordering::Less),
			(TargetOperation::Protect, _) =>
				None,
			(_, TargetOperation::Protect) =>
				None,
			
			(TargetOperation::Unlink, TargetOperation::Unlink) =>
				Some (Ordering::Greater),
			(TargetOperation::Unlink, _) =>
				None,
			(_, TargetOperation::Unlink) =>
				None,
			
			(_, _) =>
				Some (Ordering::Less),
		};
		
		if let Some (_ordering) = _ordering {
			if OsString::cmp (&_left.path, &_right.path) != _ordering {
				log_error! (0xe1ddeeda, "invalid plan for `{}`:  ordering!", _right.path_display ());
				_valid = false;
			}
		}
	}
	
	
	if _valid {
		return Ok (());
	} else {
		fail! (0xe246ff0f, "invalid plan");
	}
}




fn sift_sources (_rules : &TargetRules, _sources_existing : &EntryMap, _sources_handled : &mut PathSet, _targets_existing : &EntryMap, _targets_pending : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x9f16c940, "sifting sources...");
	
	for _source in _sources_existing.values () {
		
		let mut _handled = false;
		
		for _rule in _rules.rules.iter () {
			match _rule {
				
				
				// NOTE:
				//
				//   If the source is a folder, then recursion happens at a later stage.
				//   Also, if the source is a folder, it will later be transformed into a `MakeDir`.
				//
				//   In case of `CopyFlatten` or `CopyRename` if one selects a folder,
				//   it won't be recursively "flattened", but instead recursively "coppied" as above.
				
				TargetRule::Copy { source : _selector, target : _target } =>
					if _selector.matches (&_source) ? {
						let _descriptor = TargetDescriptor {
								path : _target.clone (),
								existing : _targets_existing.get (_target) .cloned (),
								operation : TargetOperation::Copy {
										source : _source.clone (),
									},
							};
						_targets_pending.push (_descriptor);
						_handled = true;
					}
				
				TargetRule::CopyFlatten { source : _selector, target : _target } =>
					if _selector.matches (&_source) ? {
						let _target = Path::new (_target) .join (&_source.name) .into ();
						let _descriptor = TargetDescriptor {
								existing : _targets_existing.get (&_target) .cloned (),
								path : _target,
								operation : TargetOperation::Copy {
										source : _source.clone (),
									},
							};
						_targets_pending.push (_descriptor);
						_handled = true;
					}
				
				TargetRule::CopyRename { .. } =>
					fail_unimplemented! (0xb2bb5d6d),
				
				
				// NOTE:  If  the source is a folder, it won't be recursed, but instead a symlink created pointing to it.
				
				TargetRule::Symlink { source : _selector, target : _target } =>
					if _selector.matches (&_source) ? {
						let _descriptor = TargetDescriptor {
								path : _target.clone (),
								existing : _targets_existing.get (_target) .cloned (),
								operation : TargetOperation::Symlink {
										source : _source.clone (),
									},
							};
						_targets_pending.push (_descriptor);
						_handled = true;
					}
				
				TargetRule::SymlinkFlatten { source : _selector, target : _target } =>
					if _selector.matches (&_source) ? {
						let _target = Path::new (_target) .join (&_source.name) .into ();
						let _descriptor = TargetDescriptor {
								existing : _targets_existing.get (&_target) .cloned (),
								path : _target,
								operation : TargetOperation::Symlink {
										source : _source.clone (),
									},
							};
						_targets_pending.push (_descriptor);
						_handled = true;
					}
				
				TargetRule::SymlinkRename { .. } =>
					fail_unimplemented! (0x3d416349),
				
				
				TargetRule::Protect { .. } |
				TargetRule::Unlink { .. } =>
					(),
				
				TargetRule::MakeDir { .. } |
				TargetRule::MakeSymlink { .. } =>
					(),
			}
		}
		
		if _handled {
			_sources_handled.insert (_source.path.clone ());
		}
	}
	
	return Ok (());
}




fn sift_targets (_rules : &TargetRules, _targets_existing : &EntryMap, _targets_handled : &mut PathSet, _targets_pending : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x1f72d23e, "sifting targets...");
	
	for _target in _targets_existing.values () {
		
		let mut _handled = false;
		
		for _rule in _rules.rules.iter () {
			match _rule {
				
				
				TargetRule::Protect { target : _selector } =>
					if _selector.matches (&_target) ? {
						let _descriptor = TargetDescriptor {
								path : _target.path.clone (),
								existing : Some (_target.clone ()),
								operation : TargetOperation::Protect,
							};
						_targets_pending.push (_descriptor);
						_handled = true;
						break;
					}
				
				TargetRule::Unlink { target : _selector } =>
					if _selector.matches (&_target) ? {
						let _descriptor = TargetDescriptor {
								path : _target.path.clone (),
								existing : Some (_target.clone ()),
								operation : TargetOperation::Unlink,
							};
						_targets_pending.push (_descriptor);
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
		
		if _handled {
			_targets_handled.insert (_target.path.clone ());
		}
	}
	
	return Ok (());
}




fn sift_directives (_rules : &TargetRules, _targets_existing : &EntryMap, _targets_pending : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0xcc7d8038, "sifting rules...");
	
	for _rule in _rules.rules.iter () {
		match _rule {
			
			
			TargetRule::MakeDir { target : _target } => {
				let _descriptor = TargetDescriptor {
						path : _target.clone (),
						existing : _targets_existing.get (_target) .cloned (),
						operation : TargetOperation::MakeDir,
					};
				_targets_pending.push (_descriptor);
			}
			
			TargetRule::MakeSymlink { target : _target, link : _link } => {
				let _descriptor = TargetDescriptor {
						path : _target.clone (),
						existing : _targets_existing.get (_target) .cloned (),
						operation : TargetOperation::MakeSymlink {
								link : _link.clone (),
							},
					};
				_targets_pending.push (_descriptor);
			}
			
			
			TargetRule::Protect { .. } |
			TargetRule::Unlink { .. } =>
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
	
	return Ok (());
}




fn extend_copy (_sources_existing : &EntryMap, _sources_handled : &mut PathSet, _targets_existing : &EntryMap, _targets_pending : &TargetDescriptorVec, _targets_extended : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x62d40e83, "extending copy...");
	
	for _target_1 in _targets_pending.iter () {
		
		let _source_1 = match &_target_1.operation {
			
			TargetOperation::Copy { source : _source_1 } =>
				if _source_1.is_dir {
					_source_1
				} else {
					_targets_extended.push (_target_1.clone ());
					continue;
				}
			
			_ => {
				_targets_extended.push (_target_1.clone ());
				continue;
			}
		};
		
		let _target_0 = TargetDescriptor {
				path : _target_1.path.clone (),
				existing : _target_1.existing.clone (),
				operation : TargetOperation::MakeDir,
			};
		_targets_extended.push (_target_0);
		
		for (_, _source_2) in _sources_existing.range::<OsString, _> ((Bound::Excluded (&_source_1.path), Bound::Unbounded)) {
			
			if ! Path::new (&_source_2.path) .starts_with (&_source_1.path) {
				break;
			}
			
			_sources_handled.insert (_source_2.path.clone ());
			
			let _target_2_path = Path::new (&_target_1.path) .join (Path::new (&_source_2.path) .strip_prefix (&_source_1.path) .unwrap ()) .into ();
			let _target_2 = TargetDescriptor {
					existing : _targets_existing.get (&_target_2_path) .cloned (),
					path : _target_2_path,
					operation : TargetOperation::Copy {
							source : _source_2.clone (),
						},
				};
			
			_targets_extended.push (_target_2);
		}
	}
	
	return Ok (());
}




fn extend_mkdir (_targets_existing : &EntryMap, _targets_pending : &TargetDescriptorVec, _targets_extended : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x1e7e28ce, "extending mkdir...");
	
	let mut _mkdir_include = PathSet::new ();
	let mut _mkdir_exclude = PathSet::new ();
	
	for _descriptor in _targets_pending.iter () {
		
		match _descriptor.operation {
			
			TargetOperation::Protect => {
				if let Some (_target) = &_descriptor.existing {
					if _target.is_dir && ! _target.is_symlink {
						_mkdir_exclude.remove (&_target.path);
					}
				}
				continue;
			}
			
			TargetOperation::Unlink =>
				continue,
			
			_ =>
				(),
		}
		
		for _parent in Path::new (&_descriptor.path) .ancestors () {
			let _parent = _parent.as_os_str ();
			if _parent == _descriptor.path {
				continue;
			} else if (_parent == ".") || (_parent == "..") || (_parent == "") {
				unreachable! ();
			} else {
				_mkdir_include.insert (OsString::from (_parent));
			}
		}
		
		match &_descriptor.operation {
			
			TargetOperation::MakeDir => {
				_mkdir_exclude.insert (_descriptor.path.clone ());
				continue;
			}
			
			TargetOperation::Copy { source : _source } if _source.is_dir => {
				_mkdir_exclude.insert (_descriptor.path.clone ());
				continue;
			}
			
			_ =>
				(),
		}
	}
	
	for _target in _mkdir_include.into_iter () {
		
		if _mkdir_exclude.contains (&_target) {
			continue;
		}
		
		let _descriptor = TargetDescriptor {
				path : _target.clone (),
				existing : _targets_existing.get (&_target) .cloned (),
				operation : TargetOperation::MakeDir,
			};
		
		_targets_extended.push (_descriptor);
	}
	
	return Ok (());
}




fn sort_targets (_targets_pending : TargetDescriptorVec, _targets_protect : &mut TargetDescriptorMap, _targets_unlink : &mut TargetDescriptorMap, _targets_create : &mut TargetDescriptorMap) -> Outcome<()> {
	
	log_debug! (0x1e7e28ce, "sorting descriptors...");
	
	let mut _targets_create_0 = TargetDescriptorVec::new ();
	
	for _descriptor in _targets_pending.into_iter () {
		match _descriptor.operation {
			TargetOperation::Protect =>
				if let Some (_descriptor) = _targets_protect.insert (_descriptor.path.clone (), _descriptor) {
					log_warning! (0xbfdb4bbc, "duplicate protect encountered for `{}`;  ignoring!", _descriptor.path_display ());
				}
			TargetOperation::Unlink =>
				if let Some (_descriptor) = _targets_unlink.insert (_descriptor.path.clone (), _descriptor) {
					log_warning! (0xa95058a9, "duplicate unlink encountered for `{}`;  ignoring!", _descriptor.path_display ());
				}
			_ =>
				_targets_create_0.push (_descriptor),
		}
	}
	
	for _descriptor in _targets_create_0.into_iter () {
		
		match _targets_create.entry (_descriptor.path.clone ()) {
			
			btree_map::Entry::Vacant (_cell) => {
				_cell.insert (_descriptor);
			}
			
			btree_map::Entry::Occupied (_cell) => {
				log_warning! (0x645f3787, "duplicate operation encountered for `{}`!", _cell.get () .path_display ());
				panic! (); // FIXME!
			}
		}
	}
	
	return Ok (());
}




fn prune_unlink (_targets_unlink_0 : TargetDescriptorMap, _targets_create : &TargetDescriptorMap, _targets_protect : &mut TargetDescriptorMap, _targets_unlink : &mut TargetDescriptorMap, _targets_skipped : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x067597d6, "pruning unlink...");
	
	for (_, _descriptor_unlink) in _targets_unlink_0.into_iter () .rev () {
		
		let mut _keep = true;
		let mut _protect = false;
		
		if let Some (_descriptor_create) = _targets_create.get (&_descriptor_unlink.path) {
			match &_descriptor_create.operation {
				
				TargetOperation::Protect =>
					unreachable! (),
				TargetOperation::Unlink =>
					unreachable! (),
				
				TargetOperation::Copy { source : _source } => {
					if _source.is_dir {
						unreachable! ();
					}
				}
				
				TargetOperation::Symlink { .. } => {
					if let Some (_target) = &_descriptor_unlink.existing {
						if _target.is_symlink {
							_keep = false;
						}
					}
				}
				
				TargetOperation::MakeDir => {
					if let Some (_target) = &_descriptor_unlink.existing {
						if _target.is_dir && ! _target.is_symlink {
							_keep = false;
							_protect = true;
						}
					}
				}
				
				TargetOperation::MakeSymlink { .. } => {
					if let Some (_target) = &_descriptor_unlink.existing {
						if _target.is_symlink {
							_keep = false;
						}
					}
				}
			}
		}
		
		if _protect {
			let _descriptor = TargetDescriptor {
					path : _descriptor_unlink.path.clone (),
					existing : _descriptor_unlink.existing.clone (),
					operation : TargetOperation::Protect,
				};
			if let Some (_descriptor) = _targets_protect.insert (_descriptor.path.clone (), _descriptor) {
				unreachable! ();
			}
		}
		
		if _keep {
			if let Some (_descriptor) = _targets_unlink.insert (_descriptor_unlink.path.clone (), _descriptor_unlink) {
				unreachable! ();
			}
		} else {
			_targets_skipped.push (_descriptor_unlink);
		}
	}
	
	return Ok (());
}




fn prune_create (_sources_root : &Path, _targets_root : &Path, _targets_create_0 : TargetDescriptorMap, _targets_protect : &mut TargetDescriptorMap, _targets_create : &mut TargetDescriptorMap, _targets_skipped : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x067597d6, "pruning create...");
	
	let mut _targets_pending = TargetDescriptorVec::new ();
	
	for (_, _descriptor) in _targets_create_0.into_iter () {
		match &_descriptor.operation {
			
			TargetOperation::Protect =>
				unreachable! (),
			TargetOperation::Unlink =>
				unreachable! (),
			
			TargetOperation::Copy { source : _source } => {
				_targets_pending.push (_descriptor);
			}
			
			TargetOperation::Symlink { source : _source } => {
				
				let _source_path_0 = _sources_root.join (Path::new (&_source.path) .strip_prefix ("/") .unwrap ());
				let _target_path_0 = _targets_root.join (Path::new (&_descriptor.path) .strip_prefix ("/") .unwrap ());
				
				let _link = if _source_path_0.has_root () {
					Some (_source_path_0.clone ())
				} else if let Some (_target_parent_0) = _target_path_0.parent () {
					path_diff::diff_paths (&_source_path_0, _target_parent_0)
				} else {
					None
				};
				
				let _link = if let Some (_link) = _link {
					_link.into ()
				} else {
					match fs::canonicalize (&_source_path_0) {
						Ok (_path) =>
							_path.into (),
						Err (_error) =>
							fail! (0xf9dc6544, "unexpected error encountered while canonicalizing path `{}`:  {}", _source_path_0.display (), _error),
					}
				};
				
				let _descriptor = TargetDescriptor {
						path : _descriptor.path.clone (),
						existing : _descriptor.existing.clone (),
						operation : TargetOperation::MakeSymlink {
								link : _link,
							},
					};
				_targets_pending.push (_descriptor);
			}
			
			TargetOperation::MakeDir => {
				let _skip = if let Some (_existing) = &_descriptor.existing {
					_existing.is_dir && ! _existing.is_symlink
				} else {
					false
				};
				if ! _skip {
					_targets_pending.push (_descriptor);
				} else {
					if ! _targets_protect.contains_key (&_descriptor.path) {
						let _descriptor = TargetDescriptor {
								path : _descriptor.path.clone (),
								existing : _descriptor.existing.clone (),
								operation : TargetOperation::Protect,
							};
						if let Some (_descriptor) = _targets_protect.insert (_descriptor.path.clone (), _descriptor) {
							unreachable! ();
						}
					}
					_targets_skipped.push (_descriptor);
				}
			}
			
			TargetOperation::MakeSymlink { link : _link } => {
				_targets_pending.push (_descriptor);
			}
		}
	}
	
	for _descriptor in _targets_pending.into_iter () {
		if let Some (_descriptor) = _targets_create.insert (_descriptor.path.clone (), _descriptor) {
			unreachable! ();
		}
	}
	
	return Ok (());
}

