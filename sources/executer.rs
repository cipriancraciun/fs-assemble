

use crate::prelude::*;
use crate::rules::*;




pub fn execute (
			_sources_root : &Path,
			_targets_root : &Path,
			_descriptors_planned : TargetDescriptorVec,
			_descriptors_succeeded : &mut TargetDescriptorVec,
			_descriptors_failed : &mut TargetDescriptorVec,
			_descriptors_skipped : &mut TargetDescriptorVec,
		) -> Outcome<()> {
	
	for _descriptor in _descriptors_planned.into_iter () {
		
		let _target_path_1 = Path::new (&_descriptor.path);
		let _target_path_1_parent = _target_path_1.parent ();
		let _target_path_1_display = _target_path_1.display ();
		
		let _target_path_0 = _targets_root.join (_target_path_1.strip_prefix ("/") .unwrap ());
		let _target_path_0 = &_target_path_0;
		let _target_path_0_parent = _target_path_0.parent ();
		let _target_path_0_display = _target_path_0.display ();
		
		let mut _failed = false;
		
		match &_descriptor.operation {
			
			TargetOperation::Protect => {
				match fs::symlink_metadata (_target_path_0) {
					Ok (_existing_metadata) =>
						// FIXME:  Check that the meta-data actually matches what we have indexed!
						(),
					Err (_error) => {
						log_error! (0x43c7a1ff, "failed executing protect for `{}`:  {}", _target_path_0_display, _error);
						_failed = true;
					}
				}
			},
			
			TargetOperation::Unlink => {
				if let Some (_existing) = &_descriptor.existing {
					if _existing.is_dir && ! _existing.is_symlink {
						if let Err (_error) = fs::remove_dir (_target_path_0) {
							log_error! (0x64768287, "failed executing unlink for `{}`:  {}", _target_path_0_display, _error);
							_failed = true;
						}
					} else {
						if let Err (_error) = fs::remove_file (_target_path_0) {
							log_error! (0x32536192, "failed executing unlink for `{}`:  {}", _target_path_0_display, _error);
							_failed = true;
						}
					}
				} else {
					log_error! (0xed2dc94b, "failed executing unlink for `{}`:  target does not exist", _target_path_0_display);
					_failed = true;
				}
			},
			
			TargetOperation::Copy { .. } => {
				fail_unimplemented! (0x9d504886);
			}
			
			TargetOperation::Symlink { .. } => {
				log_error! (0x33bfb4c6, "failed executing symlink for `{}`:  unsupported descriptor", _target_path_0_display);
				_failed = true;
			}
			
			TargetOperation::MakeDir => {
				if let Err (_error) = fs::create_dir (_target_path_0) {
					log_error! (0x68f2f8b2, "failed executing mkdir for `{}`:  {}", _target_path_0_display, _error);
					_failed = true;
				}
			}
			
			TargetOperation::MakeSymlink { link : _link } => {
				if let Err (_error) = fs_unix::symlink (_link, _target_path_0) {
					log_error! (0x5a1fffca, "failed executing symlink for `{}`:  {}", _target_path_0_display, _error);
					_failed = true;
				}
			}
		}
		
		if ! _failed {
			_descriptors_succeeded.push (_descriptor);
		} else {
			_descriptors_failed.push (_descriptor);
		}
	}
	
	return Ok (());
}




pub fn simplify (
			_sources_root : &Path,
			_targets_root : &Path,
			_descriptors_planned : TargetDescriptorVec,
			_descriptors_required : &mut TargetDescriptorVec,
			_descriptors_skipped : &mut TargetDescriptorVec,
		) -> Outcome<()> {
	
	for _descriptor in _descriptors_planned.into_iter () {
		
		let mut _skip = false;
		
		match &_descriptor.operation {
			
			TargetOperation::Protect =>
				if _descriptor.existing.is_none () {
					_skip = true;
				}
			
			TargetOperation::Unlink =>
				if _descriptor.existing.is_none () {
					_skip = true;
				}
			
			TargetOperation::Copy { source : _source } =>
				if _source.is_file {
					if let Some (_target) = &_descriptor.existing {
						if _target.is_file && ! _target.is_symlink {
							let _source_metadata = &_source.metadata_follow;
							let _target_metadata = &_target.metadata_symlink;
							if (_source_metadata.dev () == _target_metadata.dev ()) && (_source_metadata.ino () == _target_metadata.ino ()) {
								// NOTE:  Entities are handlinks to the same inode!
								_skip = true;
							} else if _source_metadata.size () != _target_metadata.size () {
								// NOTE:  Entities differ in size, clearly different!
							} else if _source_metadata.mtime () != _target_metadata.mtime () {
								// NOTE:  Entities differ in last modification time!
							} else {
								// FIXME:  We assume that the neither entities changed since last sync!
								_skip = true;
							}
						}
					}
				} else if _source.is_dir && ! _source.is_symlink {
					fail! (0x1686c0ee, "invalid descriptor for `{}`", _descriptor.path_display ());
				} else {
					fail! (0x0503dba7, "invalid descriptor for `{}`", _descriptor.path_display ());
				}
			
			TargetOperation::Symlink { .. } =>
				fail! (0x7406ae34, "invalid descriptor for `{}`", _descriptor.path_display ()),
			
			TargetOperation::MakeDir { .. } =>
				if let Some (_target) = &_descriptor.existing {
					if _target.is_dir && ! _target.is_symlink {
						_skip = true;
					}
				},
			
			TargetOperation::MakeSymlink { link : _link } =>
				if let Some (_target) = &_descriptor.existing {
					if _target.is_symlink && OsString::eq (_target.link.as_ref () .unwrap (), _link) {
						_skip = true;
					}
				}
		}
		
		if ! _skip {
			_descriptors_required.push (_descriptor);
		} else {
			_descriptors_skipped.push (_descriptor);
		}
	}
	
	return Ok (());
}

