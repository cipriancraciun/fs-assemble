

use crate::prelude::*;
use crate::fsas::*;




pub(crate) fn trace_script (_script : &Script, _message : Option<&str>) -> () {
	
	if let Some (_message) = _message {
		log_cut! ();
		log_debug! (0x23ea05b8, "{}", _message);
	}
	
	let mut _handled_none = true;
	
	for _statement in _script.statements.iter () {
		log_debug! (0xff407de5, "* {:?}", _statement);
		_handled_none = false;
	}
	
	if _handled_none {
		log_debug! (0x8994f4fb, "* none");
	}
	
	if _message.is_some () {
		log_cut! ();
	}
}



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




impl fmt::Debug for Entry {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		
		let mut _formatter = _formatter.debug_struct ("Entry");
		
		_formatter
				.field ("depth", &self.depth)
				.field ("path", &self.path);
		
		if self.is_dir {
			_formatter.field ("is_dir", &true);
		} else if self.is_file {
			_formatter.field ("is_file", &true);
		} else {
			_formatter.field ("is_other", &true);
		}
		
		if let Some (_link) = self.link.as_ref () {
			_formatter.field ("is_symlink", &true);
			_formatter.field ("link", _link);
		}
		
		_formatter.finish ()
	}
}




impl fmt::Debug for Pattern {
	
	fn fmt (&self, _formatter : &mut fmt::Formatter) -> fmt::Result {
		match self {
			Pattern::Exact (_pattern) =>
				_formatter.debug_tuple ("Exact") .field (_pattern) .finish (),
			Pattern::Prefix (_pattern) =>
				_formatter.debug_tuple ("Exact") .field (_pattern) .finish (),
			Pattern::Suffix (_pattern) =>
				_formatter.debug_tuple ("Exact") .field (_pattern) .finish (),
			Pattern::Glob (_pattern, _source) =>
				_formatter.debug_tuple ("Exact") .field (_source) .finish (),
			Pattern::Regex (_pattern, _source) =>
				_formatter.debug_tuple ("Exact") .field (_source) .finish (),
		}
	}
}

