

use crate::prelude::*;
use crate::rules::*;




pub(crate) fn trace_plan_create (_descriptors : &TargetDescriptorMap) -> () {
	
	log_cut! ();
	log_debug! (0x975bea76, "targets planned for creation:");
	trace_descriptors (_descriptors.values ());
	log_cut! ();
}

pub(crate) fn trace_plan_protect (_descriptors : &TargetDescriptorMap) -> () {
	
	log_cut! ();
	log_debug! (0x5fb7bc98, "targets planned for protection:");
	trace_descriptors (_descriptors.values ());
	log_cut! ();
}

pub(crate) fn trace_plan_unlink (_descriptors : &TargetDescriptorMap) -> () {
	
	log_cut! ();
	log_debug! (0xd71d0ef0, "targets planned for unlinking:");
	trace_descriptors (_descriptors.values ());
	log_cut! ();
}

pub(crate) fn trace_plan_skipped (_descriptors : &TargetDescriptorVec) -> () {
	
	log_cut! ();
	log_debug! (0x547cad62, "targets skipped:");
	trace_descriptors (_descriptors.iter ());
	log_cut! ();
}

pub(crate) fn trace_descriptors <'a> (_descriptors : impl Iterator<Item = &'a TargetDescriptor>) -> () {
	
	let mut _handled_none = true;
	
	for _descriptor in _descriptors {
		trace_descriptor (&_descriptor);
		_handled_none = false;
	}
	
	if _handled_none {
		log_debug! (0xb6addc1a, "* none");
	}
}


pub(crate) fn trace_descriptor (_descriptor : &TargetDescriptor) -> () {
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


pub(crate) fn trace_sources_unhandled (_sources_existing : &EntryMap, _sources_handled : &PathSet) -> () {
	
	log_cut! ();
	log_debug! (0xc1da0330, "sources unhandled:");
	
	let mut _handled_none = true;
	
	for _entry in _sources_existing.values () {
		if _entry.depth == 0 {
			continue;
		}
		if _sources_handled.contains (&_entry.path) {
			continue;
		}
		_handled_none = false;
		log_debug! (0xef09d9c0, "* `{}`", _entry.path_display ());
	}
	
	if _handled_none {
		log_debug! (0xbc33de37, "* none");
	}
	
	log_cut! ();
}


pub(crate) fn trace_targets_unhandled (_targets_existing : &EntryMap, _targets_handled : &PathSet) -> () {
	
	log_cut! ();
	log_debug! (0xb9728c78, "targets unhandled:");
	
	let mut _handled_none = true;
	
	for _entry in _targets_existing.values () {
		if _entry.depth == 0 {
			continue;
		}
		if _targets_handled.contains (&_entry.path) {
			continue;
		}
		_handled_none = false;
		log_debug! (0xfbb6fba3, "* `{}`", _entry.path_display ());
	}
	
	if _handled_none {
		log_debug! (0x4b943c3b, "* none");
	}
	
	log_cut! ();
}

