

use crate::prelude::*;




impl fsas::IndexFilter for () {
	
	fn filter (&self, _entry : &fsas::Entry) -> Outcome<fsas::IndexDecision> {
		Ok (fsas::IndexDecision { collect : true, recurse : true })
	}
}




impl fsas::FilterRules {
	
	pub fn new () -> Self {
		
		return Self {
				rules : Vec::new (),
				skip_hidden : true,
				recurse_symlinks : false,
			};
	}
}




impl fsas::IndexFilter for fsas::FilterRules {
	
	fn filter (&self, _entry : &fsas::Entry) -> Outcome<fsas::IndexDecision> {
		
		if (_entry.name.len () > 1) && (_entry.name.as_bytes () [0] == b'.') {
			if self.skip_hidden {
				return Ok (fsas::IndexDecision { collect : false, recurse : false });
			}
		}
		
		let mut _collect = true;
		let mut _recurse = true;
		
		for _rule in &self.rules {
			match _rule.filter (_entry) ? {
				Some (true) =>
					break,
				Some (false) => {
					_collect = false;
					_recurse = false;
					break;
				}
				None =>
					(),
			}
		}
		
		if ! self.recurse_symlinks && _entry.is_symlink () && _entry.is_dir () {
			_recurse = false;
		}
		
		return Ok (fsas::IndexDecision { collect : _collect, recurse : _recurse });
	}
}




impl fsas::FilterRule {
	
	pub fn filter (&self, _entry : &fsas::Entry) -> Outcome<Option<bool>> {
		
		match self {
			
			fsas::FilterRule::Include { selector : _selector } =>
				if _selector.matches (_entry) ? {
					Ok (Some (true))
				} else {
					Ok (None)
				},
			
			fsas::FilterRule::Exclude { selector : _selector } =>
				if _selector.matches (_entry) ? {
					Ok (Some (false))
				} else {
					Ok (None)
				},
		}
	}
}


impl fsas::Selector {
	
	pub fn matches (&self, _entry : &fsas::Entry) -> Outcome<bool> {
		
		match self {
			
			fsas::Selector::Exact { pattern : _pattern, name_only : _name_only } =>
				if *_name_only {
					Ok (OsStr::eq (_pattern.as_ref (), &_entry.name))
				} else {
					Ok (OsStr::eq (_pattern.as_ref (), &_entry.path))
				},
			
			_ =>
				fail! (0x658d0967, "not implemented"),
		}
	}
}

