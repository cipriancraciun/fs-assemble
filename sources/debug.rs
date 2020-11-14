

use crate::prelude::*;
use crate::rules::*;




pub(crate) fn trace_descriptors <'a> (_descriptors : impl Iterator<Item = &'a TargetDescriptor>, _message : Option<&str>) -> () {
	
	if let Some (_message) = _message {
		log_cut! ();
		log_debug! (0x16bda71d, "{}", _message);
	}
	
	let mut _handled_none = true;
	
	for _descriptor in _descriptors {
		trace_descriptor (&_descriptor);
		_handled_none = false;
	}
	
	if _handled_none {
		log_debug! (0xb6addc1a, "* none");
	}
	
	if _message.is_some () {
		log_cut! ();
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




pub(crate) fn trace_entries <'a> (_entries : impl Iterator<Item = &'a Entry>, _message : Option<&str>) -> () {
	
	if let Some (_message) = _message {
		log_cut! ();
		log_debug! (0xc1da0330, "{}", _message);
	}
	
	let mut _handled_none = true;
	
	for _entry in _entries {
		if _entry.depth == 0 {
			continue;
		}
		_handled_none = false;
		log_debug! (0xef09d9c0, "* `{}`", _entry.path_display ());
	}
	
	if _handled_none {
		log_debug! (0xbc33de37, "* none");
	}
	
	if _message.is_some () {
		log_cut! ();
	}
}

