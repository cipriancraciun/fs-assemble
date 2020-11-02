

use crate::prelude::*;




impl fsas::FilterRules {
	
	pub fn new () -> Self {
		
		return Self {
				rules : Vec::new (),
				symlinks_collect : true,
				symlinks_recurse : true,
				hidden_collect : false,
				hidden_recurse : false,
			};
	}
}




impl fsas::IndexFilter for fsas::FilterRules {
	
	fn filter (&self, _entry : &fsas::Entry) -> Outcome<fsas::IndexDecision> {
		
		let mut _collect = true;
		let mut _recurse = true;
		
		if _entry.is_symlink {
			_collect &= self.symlinks_collect;
			_collect &= self.symlinks_recurse;
		}
		
		if _entry.is_hidden {
			_collect &= self.hidden_collect;
			_recurse &= self.hidden_recurse;
		}
		
		if _collect || _recurse {
			for _rule in &self.rules {
				match _rule.filter (_entry) ? {
					Some (true) =>
						break,
					Some (false) => {
						_collect &= false;
						_recurse &= false;
						break;
					}
					None =>
						(),
				}
			}
		}
		
		let _decision = fsas::IndexDecision {
				collect : _collect,
				recurse : _recurse,
			};
		
		return Ok (_decision);
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

