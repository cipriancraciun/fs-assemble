

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
		
		if ! self.recurse_symlinks && _entry.is_symlink () && _entry.is_dir () {
			_recurse &= false;
		}
		
		return Ok (fsas::IndexDecision { collect : _collect, recurse : _recurse });
	}
}

