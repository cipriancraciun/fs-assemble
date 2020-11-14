

use crate::prelude::*;
use crate::rules::*;




pub fn plan (_rules : &TargetRules, _sources_root : &Path, _sources : EntryVec, _targets_root : &Path, _targets : EntryVec) -> Outcome<TargetDescriptorVec> {
	
	
	let mut _sources_existing = fsas::build_tree (_sources) ?;
	let mut _sources_handled = PathSet::new ();
	
	let mut _targets_existing = fsas::build_tree (_targets) ?;
	let mut _targets_handled = PathSet::new ();
	
	
	let mut _targets_planned = TargetDescriptorVec::new ();
	
	sift_sources (_rules, &_sources_existing, &mut _sources_handled, &_targets_existing, &mut _targets_planned) ?;
	sift_targets (_rules, &_targets_existing, &mut _targets_handled, &mut _targets_planned) ?;
	sift_directives (_rules, &_targets_existing, &mut _targets_planned) ?;
	
	let mut _targets_extended = TargetDescriptorVec::new ();
	extend_copy (&_sources_existing, &mut _sources_handled, &_targets_existing, &_targets_planned, &mut _targets_extended) ?;
	_targets_planned.extend (_targets_extended.into_iter ());
	
	let mut _targets_extended = TargetDescriptorVec::new ();
	extend_mkdir (&_targets_existing, &_targets_planned, &mut _targets_extended) ?;
	_targets_planned.extend (_targets_extended.into_iter ());
	
	let mut _targets_protect = TargetDescriptorMap::new ();
	let mut _targets_unlink = TargetDescriptorMap::new ();
	let mut _targets_pending = TargetDescriptorMap::new ();
	
	sort_targets (_targets_planned, &mut _targets_protect, &mut _targets_unlink, &mut _targets_pending) ?;
	
	let mut _targets_planned = TargetDescriptorVec::new ();
	let mut _targets_skipped = TargetDescriptorVec::new ();
	
	prune_unlink (_targets_unlink, &_targets_protect, &_targets_pending, &mut _targets_planned, &mut _targets_skipped) ?;
	prune_pending (_sources_root, _targets_root, _targets_pending, &_targets_protect, &mut _targets_planned, &mut _targets_skipped) ?;
	
	
	trace_planned (&_targets_planned);
	trace_protected (&_targets_protect);
	trace_skipped (&_targets_skipped);
	trace_sources_unhandled (&_sources_existing, &_sources_handled);
	trace_targets_unhandled (&_targets_existing, &_targets_handled);
	
	
	_targets_planned.extend (_targets_protect.into_iter () .map (|(_, _descriptor)| _descriptor));
	
	
	return Ok (_targets_planned);
}




type EntryVec = Vec<Entry>;
type EntryMap = BTreeMap<OsString, Entry>;

// type PathVec = Vec<OsString>;
type PathSet = BTreeSet<OsString>;

type TargetDescriptorVec = Vec<TargetDescriptor>;
type TargetDescriptorMap = BTreeMap<OsString, TargetDescriptor>;




fn sift_sources (_rules : &TargetRules, _sources_existing : &EntryMap, _sources_handled : &mut PathSet, _targets_existing : &EntryMap, _targets_planned : &mut TargetDescriptorVec) -> Outcome<()> {
	
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
						_targets_planned.push (_descriptor);
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
						_targets_planned.push (_descriptor);
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
						_targets_planned.push (_descriptor);
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
						_targets_planned.push (_descriptor);
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




fn sift_targets (_rules : &TargetRules, _targets_existing : &EntryMap, _targets_handled : &mut PathSet, _targets_planned : &mut TargetDescriptorVec) -> Outcome<()> {
	
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
						_targets_planned.push (_descriptor);
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
						_targets_planned.push (_descriptor);
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




fn sift_directives (_rules : &TargetRules, _targets_existing : &EntryMap, _targets_planned : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0xcc7d8038, "sifting directives...");
	
	for _rule in _rules.rules.iter () {
		match _rule {
			
			
			TargetRule::MakeDir { target : _target } => {
				let _descriptor = TargetDescriptor {
						path : _target.clone (),
						existing : _targets_existing.get (_target) .cloned (),
						operation : TargetOperation::MakeDir,
					};
				_targets_planned.push (_descriptor);
			}
			
			TargetRule::MakeSymlink { target : _target, link : _link } => {
				let _descriptor = TargetDescriptor {
						path : _target.clone (),
						existing : _targets_existing.get (_target) .cloned (),
						operation : TargetOperation::MakeSymlink {
								link : _link.clone (),
							},
					};
				_targets_planned.push (_descriptor);
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




fn extend_copy (_sources_existing : &EntryMap, _sources_handled : &mut PathSet, _targets_existing : &EntryMap, _targets_planned : &TargetDescriptorVec, _targets_extended : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x62d40e83, "extending copy...");
	
	for _target_1 in _targets_planned.iter () {
		
		let _source_1 = match &_target_1.operation {
			
			TargetOperation::Copy { source : _source_1 } =>
				if _source_1.is_dir {
					_source_1
				} else {
					continue;
				}
			
			_ =>
				continue,
		};
		
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




fn extend_mkdir (_targets_existing : &EntryMap, _targets_planned : &TargetDescriptorVec, _targets_extended : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x1e7e28ce, "extending mkdir...");
	
	let mut _mkdir_include = PathSet::new ();
	let mut _mkdir_exclude = PathSet::new ();
	
	for _descriptor in _targets_planned.iter () {
		
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




fn sort_targets (_targets_planned : TargetDescriptorVec, _targets_protect : &mut TargetDescriptorMap, _targets_unlink : &mut TargetDescriptorMap, _targets_pending : &mut TargetDescriptorMap) -> Outcome<()> {
	
	log_debug! (0x1e7e28ce, "sorting targets...");
	
	let mut _targets_pending_0 = TargetDescriptorVec::new ();
	
	for _descriptor in _targets_planned.into_iter () {
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
				_targets_pending_0.push (_descriptor),
		}
	}
	
	for _descriptor in _targets_pending_0.into_iter () {
		
		match _targets_pending.entry (_descriptor.path.clone ()) {
			
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




fn prune_unlink (_targets_unlink : TargetDescriptorMap, _targets_protect : &TargetDescriptorMap, _targets_pending : &TargetDescriptorMap, _targets_planned : &mut TargetDescriptorVec, _targets_skipped : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x067597d6, "pruning unlink...");
	
	for (_, _descriptor_unlink) in _targets_unlink.into_iter () .rev () {
		
		let mut _keep = true;
		
		if let Some (_descriptor_protect) = _targets_protect.get (&_descriptor_unlink.path) {
			log_error! (0x908583c1, "conflicting operations for path `{}`:  unlinked and protected;", _descriptor_unlink.path_display ());
			fail! (0x7c1c742f, "conflicting operations for path `{}`", _descriptor_unlink.path_display ());
		}
		
		if let Some (_descriptor_pending) = _targets_pending.get (&_descriptor_unlink.path) {
			match &_descriptor_pending.operation {
				
				TargetOperation::Protect =>
					unreachable! (),
				TargetOperation::Unlink =>
					unreachable! (),
				
				TargetOperation::Copy { source : _source } => {
					if _source.is_dir {
						if let Some (_target) = &_descriptor_unlink.existing {
							if _target.is_dir && ! _target.is_symlink && _source.is_dir {
								_keep = false;
							}
						}
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
		
		if _keep {
			_targets_planned.push (_descriptor_unlink);
		} else {
			_targets_skipped.push (_descriptor_unlink);
		}
	}
	
	return Ok (());
}




fn prune_pending (_sources_root : &Path, _targets_root : &Path, _targets_pending : TargetDescriptorMap, _targets_protect : &TargetDescriptorMap, _targets_planned : &mut TargetDescriptorVec, _targets_skipped : &mut TargetDescriptorVec) -> Outcome<()> {
	
	log_debug! (0x067597d6, "pruning pending...");
	
	for (_, _descriptor) in _targets_pending.into_iter () {
		match &_descriptor.operation {
			
			TargetOperation::Protect =>
				unreachable! (),
			TargetOperation::Unlink =>
				unreachable! (),
			
			TargetOperation::Copy { source : _source } => {
				_targets_planned.push (_descriptor);
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
				
				let _skip = if let Some (_existing) = &_descriptor.existing {
					_existing.is_symlink && OsString::eq (_existing.link.as_ref () .unwrap (), &_link)
				} else {
					false
				};
				
				if _skip {
					_targets_skipped.push (_descriptor);
				} else {
					let _descriptor = TargetDescriptor {
							path : _descriptor.path.clone (),
							existing : _descriptor.existing.clone (),
							operation : TargetOperation::MakeSymlink {
									link : _link,
								},
						};
					_targets_planned.push (_descriptor);
				}
			}
			
			TargetOperation::MakeDir => {
				if let Some (_existing) = &_descriptor.existing {
					if _existing.is_dir && ! _existing.is_symlink {
						_targets_skipped.push (_descriptor);
					} else {
						_targets_planned.push (_descriptor);
					}
				} else {
					_targets_planned.push (_descriptor);
				}
			}
			
			TargetOperation::MakeSymlink { link : _link } => {
				if let Some (_existing) = &_descriptor.existing {
					if _existing.is_symlink && OsString::eq (_existing.link.as_ref () .unwrap (), _link) {
						_targets_skipped.push (_descriptor);
					} else {
						_targets_planned.push (_descriptor);
					}
				} else {
					_targets_planned.push (_descriptor);
				}
			}
		}
	}
	
	return Ok (());
}




fn trace_planned (_targets_planned : &TargetDescriptorVec) -> () {
	
	log_cut! ();
	log_debug! (0x975bea76, "targets planned:");
	trace_descriptors (_targets_planned.iter ());
	log_cut! ();
}


fn trace_protected (_targets_protected : &TargetDescriptorMap) -> () {
	
	log_cut! ();
	log_debug! (0x547cad62, "targets protected:");
	trace_descriptors (_targets_protected.values ());
	log_cut! ();
}


fn trace_skipped (_targets_skipped : &TargetDescriptorVec) -> () {
	
	log_cut! ();
	log_debug! (0x547cad62, "targets skipped:");
	trace_descriptors (_targets_skipped.iter ());
	log_cut! ();
}


fn trace_descriptors <'a> (_descriptors : impl Iterator<Item = &'a TargetDescriptor>) -> () {
	
	let mut _handled_all = true;
	
	for _descriptor in _descriptors {
		trace_descriptor (&_descriptor);
	}
	
	if _handled_all {
		log_debug! (0xb6addc1a, "* none");
	}
}


fn trace_descriptor (_descriptor : &TargetDescriptor) -> () {
	match &_descriptor.operation {
		TargetOperation::Protect =>
			log_debug! (0xf0141374, "* protect `{}`", _descriptor.path_display ()),
		TargetOperation::Unlink =>
			log_debug! (0x096428c7, "* unlink `{}`", _descriptor.path_display ()),
		TargetOperation::Copy { source : _source } =>
			log_debug! (0xbd64ca66, "* copy `{}` from `{}`", _descriptor.path_display (), _source.path_display ()),
		TargetOperation::Symlink { source : _source } =>
			log_debug! (0x6aa9b259, "* symlink `{}` from `{}`", _descriptor.path_display (), _source.path_display ()),
		TargetOperation::MakeDir =>
			log_debug! (0xa5485064, "* mkdir `{}`", _descriptor.path_display ()),
		TargetOperation::MakeSymlink { link : _link } =>
			log_debug! (0x27c9eb12, "* symlink `{}` to `{}`", _descriptor.path_display (), Path::new (_link) .display ()),
	}
}


fn trace_sources_unhandled (_sources_existing : &EntryMap, _sources_handled : &PathSet) -> () {
	
	log_cut! ();
	log_debug! (0xc1da0330, "sources unhandled:");
	
	let mut _handled_all = true;
	
	for _entry in _sources_existing.values () {
		if _entry.depth == 0 {
			continue;
		}
		if _sources_handled.contains (&_entry.path) {
			continue;
		}
		_handled_all = false;
		log_debug! (0xef09d9c0, "* `{}`", _entry.path_display ());
	}
	
	if _handled_all {
		log_debug! (0xbc33de37, "* none");
	}
	
	log_cut! ();
}


fn trace_targets_unhandled (_targets_existing : &EntryMap, _targets_handled : &PathSet) -> () {
	
	log_cut! ();
	log_debug! (0xb9728c78, "targets unhandled:");
	
	let mut _handled_all = true;
	
	for _entry in _targets_existing.values () {
		if _entry.depth == 0 {
			continue;
		}
		if _targets_handled.contains (&_entry.path) {
			continue;
		}
		_handled_all = false;
		log_debug! (0xfbb6fba3, "* `{}`", _entry.path_display ());
	}
	
	if _handled_all {
		log_debug! (0x4b943c3b, "* none");
	}
	
	log_cut! ();
}

