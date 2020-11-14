

use crate::prelude::*;
use crate::rules::*;




pub fn execute (
			_sources_root : &Path,
			_targets_root : &Path,
			_descriptors_planned : TargetDescriptorVec,
			_descriptors_succeeded : &mut TargetDescriptorVec,
			_descriptors_failed : &mut TargetDescriptorVec,
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
				(),
			TargetOperation::Unlink =>
				(),
			
			TargetOperation::Copy { .. } =>
				(),
			TargetOperation::Symlink { .. } =>
				fail! (0x7406ae34, "invalid descriptor for `{}`", _descriptor.path_display ()),
			
			TargetOperation::MakeDir { .. } =>
				(),
			TargetOperation::MakeSymlink { .. } =>
				(),
		}
		
		if ! _skip {
			_descriptors_required.push (_descriptor);
		} else {
			_descriptors_skipped.push (_descriptor);
		}
	}
	
	return Ok (());
}

