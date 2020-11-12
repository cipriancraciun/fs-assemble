

use crate::prelude::*;




impl fsas::IndexFilter for fsas::IndexRules {
	
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




impl fsas::IndexRule {
	
	pub fn filter (&self, _entry : &fsas::Entry) -> Outcome<Option<bool>> {
		
		match self {
			
			fsas::IndexRule::Include { selector : _selector } =>
				if _selector.matches (_entry) ? {
					Ok (Some (true))
				} else {
					Ok (None)
				},
			
			fsas::IndexRule::Exclude { selector : _selector } =>
				if _selector.matches (_entry) ? {
					Ok (Some (false))
				} else {
					Ok (None)
				},
		}
	}
}




impl fsas::EntrySelector {
	
	pub fn matches (&self, _entry : &fsas::Entry) -> Outcome<bool> {
		
		match self {
			
			fsas::EntrySelector::Always =>
				return Ok (true),
			
			fsas::EntrySelector::Never =>
				return Ok (false),
			
			fsas::EntrySelector::Matches (_matcher) =>
				return _matcher.matches (_entry),
			
			fsas::EntrySelector::NotMatches (_matcher) =>
				return _matcher.matches (_entry) .map (|_matches| !_matches),
			
			fsas::EntrySelector::Any (_selectors) => {
				for _selector in _selectors {
					if _selector.matches (_entry) ? {
						return Ok (true);
					}
				}
				return Ok (false);
			}
			
			fsas::EntrySelector::All (_selectors) => {
				for _selector in _selectors {
					if ! _selector.matches (_entry) ? {
						return Ok (false);
					}
				}
				return Ok (true);
			}
			
			fsas::EntrySelector::None (_selectors) => {
				for _selector in _selectors {
					if _selector.matches (_entry) ? {
						return Ok (false);
					}
				}
				return Ok (true);
			}
		}
	}
}




impl fsas::EntryMatcher {
	
	pub fn matches (&self, _entry : &fsas::Entry) -> Outcome<bool> {
		
		match self {
			
			fsas::EntryMatcher::Path (_pattern) =>
				_pattern.matches (&_entry.path),
			fsas::EntryMatcher::Name (_pattern) =>
				_pattern.matches (&_entry.name),
			
			fsas::EntryMatcher::IsSymlink =>
				Ok (_entry.is_symlink),
			fsas::EntryMatcher::IsDir =>
				Ok (_entry.is_dir),
			fsas::EntryMatcher::IsFile =>
				Ok (_entry.is_file),
			fsas::EntryMatcher::IsHidden =>
				Ok (_entry.is_hidden),
		}
	}
}




impl fsas::Pattern {
	
	pub fn matches (&self, _input : &OsStr) -> Outcome<bool> {
		
		match self {
			
			fsas::Pattern::Exact (_pattern) =>
				Ok (OsStr::eq (_input, _pattern)),
			fsas::Pattern::Prefix (_pattern) =>
				Ok (_input.as_bytes () .starts_with (_pattern.as_bytes ())),
			fsas::Pattern::Suffix (_pattern) =>
				Ok (_input.as_bytes () .ends_with (_pattern.as_bytes ())),
			
			fsas::Pattern::Glob (_pattern) =>
				// FIXME:  Use `globset::Candidate` to amortize preprocessing cost!
				Ok (_pattern.is_match (_input)),
			
			fsas::Pattern::Regex (_pattern) =>
				Ok (_pattern.is_match (_input.as_bytes ())),
			
		}
	}
}

