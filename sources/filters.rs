

use crate::prelude::*;
use crate::rules::*;




impl IndexFilter for IndexRules {
	
	fn filter (&self, _entry : &Entry) -> Outcome<IndexDecision> {
		
		let mut _collect = self.fallback_collect;
		let mut _recurse = self.fallback_recurse;
		
		if _entry.is_symlink {
			_collect &= self.symlinks_collect;
			_recurse &= self.symlinks_recurse;
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
		
		let _decision = IndexDecision {
				collect : _collect,
				recurse : _recurse,
			};
		
		return Ok (_decision);
	}
}




impl IndexRule {
	
	pub fn filter (&self, _entry : &Entry) -> Outcome<Option<bool>> {
		
		match self {
			
			IndexRule::Include { selector : _selector } =>
				if _selector.matches (_entry) ? {
					Ok (Some (true))
				} else {
					Ok (None)
				},
			
			IndexRule::Exclude { selector : _selector } =>
				if _selector.matches (_entry) ? {
					Ok (Some (false))
				} else {
					Ok (None)
				},
		}
	}
}




impl EntrySelector {
	
	pub fn matches (&self, _entry : &Entry) -> Outcome<bool> {
		
		match self {
			
			EntrySelector::Always =>
				return Ok (true),
			
			EntrySelector::Never =>
				return Ok (false),
			
			EntrySelector::Matches (_matcher) =>
				return _matcher.matches (_entry),
			
			EntrySelector::NotMatches (_matcher) =>
				return _matcher.matches (_entry) .map (|_matches| !_matches),
			
			EntrySelector::Any (_selectors) => {
				for _selector in _selectors {
					if _selector.matches (_entry) ? {
						return Ok (true);
					}
				}
				return Ok (false);
			}
			
			EntrySelector::All (_selectors) => {
				for _selector in _selectors {
					if ! _selector.matches (_entry) ? {
						return Ok (false);
					}
				}
				return Ok (true);
			}
			
			EntrySelector::None (_selectors) => {
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




impl EntryMatcher {
	
	pub fn matches (&self, _entry : &Entry) -> Outcome<bool> {
		
		match self {
			
			EntryMatcher::Path (_pattern) =>
				_pattern.matches (&_entry.path),
			EntryMatcher::Name (_pattern) =>
				_pattern.matches (&_entry.name),
			
			EntryMatcher::IsSymlink =>
				Ok (_entry.is_symlink),
			EntryMatcher::IsDir =>
				Ok (_entry.is_dir),
			EntryMatcher::IsFile =>
				Ok (_entry.is_file),
			EntryMatcher::IsHidden =>
				Ok (_entry.is_hidden),
			
			EntryMatcher::IsNotSymlink =>
				Ok (! _entry.is_symlink),
			EntryMatcher::IsNotDir =>
				Ok (! _entry.is_dir),
			EntryMatcher::IsNotFile =>
				Ok (! _entry.is_file),
			EntryMatcher::IsNotHidden =>
				Ok (! _entry.is_hidden),
		}
	}
}




impl Pattern {
	
	pub fn matches (&self, _input : &OsStr) -> Outcome<bool> {
		
		match self {
			
			Pattern::Exact (_pattern) =>
				Ok (OsStr::eq (_input, _pattern)),
			Pattern::Prefix (_pattern) =>
				Ok (_input.as_bytes () .starts_with (_pattern.as_bytes ())),
			Pattern::Suffix (_pattern) =>
				Ok (_input.as_bytes () .ends_with (_pattern.as_bytes ())),
			
			Pattern::Glob (_pattern, _) =>
				// FIXME:  Use `globset::Candidate` to amortize preprocessing cost!
				Ok (_pattern.is_match (_input)),
			
			Pattern::Regex (_pattern, _) =>
				Ok (_pattern.is_match (_input.as_bytes ())),
			
		}
	}
}

