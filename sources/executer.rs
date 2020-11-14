

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
	
	fsas::trace_descriptors (_descriptors_planned.iter (), Some ("descriptors planned for execution:"));
	
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

