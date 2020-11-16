

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
		let _target_path_0 = _targets_root.join (_target_path_1.strip_prefix ("/") .unwrap ());
		let _target_path_0 = &_target_path_0;
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
						if EXECUTION_ALLOWED {
							if let Err (_error) = fs::remove_dir (_target_path_0) {
								log_error! (0x64768287, "failed executing unlink for `{}`:  {}", _target_path_0_display, _error);
								_failed = true;
							}
						}
					} else {
						if EXECUTION_ALLOWED {
							if let Err (_error) = fs::remove_file (_target_path_0) {
								log_error! (0x32536192, "failed executing unlink for `{}`:  {}", _target_path_0_display, _error);
								_failed = true;
							}
						}
					}
				} else {
					log_error! (0xed2dc94b, "failed executing unlink for `{}`:  target does not exist", _target_path_0_display);
					_failed = true;
				}
			},
			
			TargetOperation::Copy { source : _source } => {
				
				let _source_path_1 = Path::new (&_source.path);
				let _source_path_0 = _sources_root.join (_source_path_1.strip_prefix ("/") .unwrap ());
				let _source_path_0 = &_source_path_0;
				
				_failed = ! execute_copy (_target_path_0, _source_path_0, _source) ?;
			}
			
			TargetOperation::Symlink { .. } => {
				log_error! (0x33bfb4c6, "failed executing symlink for `{}`:  unsupported descriptor", _target_path_0_display);
				_failed = true;
			}
			
			TargetOperation::MakeDir => {
				if EXECUTION_ALLOWED {
					if let Err (_error) = fs::create_dir (_target_path_0) {
						log_error! (0x68f2f8b2, "failed executing mkdir for `{}`:  {}", _target_path_0_display, _error);
						_failed = true;
					}
				}
			}
			
			TargetOperation::MakeSymlink { link : _link } => {
				if EXECUTION_ALLOWED {
					// FIXME:  Plan an unlink instead of this!
					if let Some (_existing) = &_descriptor.existing {
						if _existing.is_symlink {
							if let Err (_error) = fs::remove_file (_target_path_0) {
								log_error! (0xc69b29f0, "failed executing symlink for `{}`:  {}", _target_path_0_display, _error);
								_failed = true;
							}
						}
					}
					if let Err (_error) = fs_unix::symlink (_link, _target_path_0) {
						log_error! (0x5a1fffca, "failed executing symlink for `{}`:  {}", _target_path_0_display, _error);
						_failed = true;
					}
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




pub(crate) fn execute_copy (_target_path_0 : &Path, _source_path_0 : &Path, _source : &Entry) -> Outcome<bool> {
	
	let _target_path_0_display = _target_path_0.display ();
	let _source_path_0_display = _source_path_0.display ();
	
	if _source.is_file {
		
		if ! EXECUTION_ALLOWED {
			return Ok (true);
		}
		
		let mut _temp_builder = tempfile::Builder::new ();
		_temp_builder
				.prefix (TEMPFILE_PREFIX)
				.suffix (TEMPFILE_SUFFIX)
				.rand_bytes (TEMPFILE_TOKEN);
		
		let _temp_file = if let Some (_temp_parent_0) = _target_path_0.parent () {
			match _temp_builder.tempfile_in (_temp_parent_0) {
				Ok (_temp_file) =>
					_temp_file.into_temp_path (),
				Err (_error) => {
					log_error! (0xb88c5290, "failed executing copy for `{}`:  {}", _target_path_0_display, _error);
					return Ok (false);
				}
			}
		} else {
			log_error! (0xad7e37a1, "failed executing copy for `{}`:  invalid parent", _target_path_0_display);
			return Ok (false);
		};
		
		let _target_temp_0 : &Path = _temp_file.as_ref ();
		
		if let Err (_error) = fs::copy (_source_path_0, _target_temp_0) {
			log_error! (0x95cab520, "failed executing copy for `{}` from `{}`:  {}", _target_path_0_display, _source_path_0_display, _error);
			return Ok (false);
		}
		
		let _target_atime = nix::TimeValLike::nanoseconds ((_source.metadata_follow.atime () * 1_000_000_000) + _source.metadata_follow.atime_nsec ());
		let _target_mtime = nix::TimeValLike::nanoseconds ((_source.metadata_follow.mtime () * 1_000_000_000) + _source.metadata_follow.mtime_nsec ());
		if let Err (_error) = nix::lutimes (_target_temp_0, &_target_atime, &_target_mtime) {
			log_error! (0x6e0b1ee3, "failed executing utimes for `{}`:  {}", _target_path_0_display, _error);
			return Ok (false);
		}
		
		if let Err (_error) = fs::rename (_target_temp_0, _target_path_0) {
			log_error! (0xb146252c, "failed executing rename for `{}`:  {}", _target_path_0_display, _error);
			return Ok (false);
		}
		
		if let Err (_error) = _temp_file.keep () {
			log_error! (0x5125a953, "failed executing copy for `{}`:  {}", _target_path_0_display, _error);
			return Ok (false);
		}
		
		return Ok (true);
		
	} else {
		log_error! (0x21a927bf, "failed executing copy for `{}` from `{}`:  unsupported source", _target_path_0_display, _source_path_0_display);
		return Ok (false);
	}
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

