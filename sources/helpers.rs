

use crate::prelude::*;
use crate::rules::*;




impl IndexRules {
	
	
	pub fn new_for_source () -> Self {
		
		return Self {
				rules : Vec::new (),
				symlinks_collect : true,
				symlinks_recurse : true,
				hidden_collect : false,
				hidden_recurse : false,
				fallback_collect : true,
				fallback_recurse : true,
			};
	}
	
	pub fn new_for_target () -> Self {
		
		return Self {
				rules : Vec::new (),
				symlinks_collect : true,
				symlinks_recurse : false,
				hidden_collect : true,
				hidden_recurse : true,
				fallback_collect : true,
				fallback_recurse : true,
			};
	}
	
	
	pub fn push_include (&mut self, _selector : EntrySelector) -> &mut Self {
		self.rules.push (IndexRule::include (_selector));
		self
	}
	
	pub fn push_exclude (&mut self, _selector : EntrySelector) -> &mut Self {
		self.rules.push (IndexRule::exclude (_selector));
		self
	}
}




impl IndexRule {
	
	
	pub fn include (_selector : EntrySelector) -> Self {
		IndexRule::Include { selector : _selector }
	}
	
	pub fn exclude (_selector : EntrySelector) -> Self {
		IndexRule::Exclude { selector : _selector }
	}
}




impl EntrySelector {
	
	
	pub fn negate (self) -> EntrySelector {
		match self {
			
			EntrySelector::Always =>
				EntrySelector::Never,
			EntrySelector::Never =>
				EntrySelector::Always,
			
			EntrySelector::Matches (_matcher) =>
				EntrySelector::NotMatches (_matcher),
			EntrySelector::NotMatches (_matcher) =>
				EntrySelector::Matches (_matcher),
			
			EntrySelector::Any (_selectors) =>
				EntrySelector::All (_selectors.into_iter () .map (EntrySelector::negate) .collect ()),
			EntrySelector::All (_selectors) =>
				EntrySelector::Any (_selectors.into_iter () .map (EntrySelector::negate) .collect ()),
			
			EntrySelector::None (_selectors) =>
				EntrySelector::Any (_selectors),
		}
	}
	
	
	pub fn if_matches_path (_pattern : Pattern) -> EntrySelector {
		EntrySelector::Matches (EntryMatcher::Path (_pattern))
	}
	
	pub fn if_matches_name (_pattern : Pattern) -> EntrySelector {
		EntrySelector::Matches (EntryMatcher::Name (_pattern))
	}
	
	pub fn if_is_symlink () -> EntrySelector {
		EntrySelector::Matches (EntryMatcher::IsSymlink)
	}
	
	pub fn if_is_dir () -> EntrySelector {
		EntrySelector::Matches (EntryMatcher::IsDir)
	}
	
	pub fn if_is_file () -> EntrySelector {
		EntrySelector::Matches (EntryMatcher::IsFile)
	}
	
	pub fn if_is_hidden () -> EntrySelector {
		EntrySelector::Matches (EntryMatcher::IsHidden)
	}
	
	
	pub fn then_include (self) -> IndexRule {
		IndexRule::include (self)
	}
	
	pub fn then_exclude (self) -> IndexRule {
		IndexRule::exclude (self)
	}
}




impl EntryMatcher {
	
	
	pub fn if_matches (self) -> EntrySelector {
		EntrySelector::Matches (self)
	}
	
	pub fn if_not_matches (self) -> EntrySelector {
		EntrySelector::NotMatches (self)
	}
	
	
	pub fn then_include (self) -> IndexRule {
		self.if_matches () .then_include ()
	}
	
	pub fn then_exclude (self) -> IndexRule {
		self.if_matches () .then_exclude ()
	}
}




impl Pattern {
	
	
	pub fn exact (_pattern : &str) -> Self {
		Pattern::Exact (_pattern.into ())
	}
	
	
	pub fn prefix (_pattern : &str) -> Self {
		Pattern::Prefix (_pattern.into ())
	}
	
	
	pub fn suffix (_pattern : &str) -> Self {
		Pattern::Suffix (_pattern.into ())
	}
	
	
	pub fn glob (_pattern : &str) -> Outcome<Self> {
		
		let mut _builder = globset::GlobSetBuilder::new ();
		
		match globset::Glob::new (_pattern) {
			Ok (_pattern) => {
				_builder.add (_pattern);
			}
			Err (_error) =>
				fail! (0xf4de3d8d, "invalid glob pattern `{}`:  {}", _pattern, _error),
		}
		
		match _builder.build () {
			Ok (_pattern) =>
				Ok (Pattern::Glob (_pattern)),
			Err (_error) =>
				fail! (0xdca40b00, "unexpected glob error:  {}", _error),
		}
	}
	
	
	pub fn regex (_pattern : &str) -> Outcome<Self> {
		
		let mut _builder = regexb::RegexSetBuilder::new (&[_pattern]);
		
		_builder.case_insensitive (false);
		_builder.multi_line (false);
		_builder.dot_matches_new_line (true);
		_builder.unicode (true);
		
		match _builder.build () {
			Ok (_pattern) =>
				Ok (Pattern::Regex (_pattern)),
			Err (_error) =>
				fail! (0x6f0bb71e, "invalid regex pattern `{}`:  {}", _pattern, _error),
		}
	}
}




impl TargetRules {
	
	
	pub fn new () -> Self {
		return Self {
				rules : Vec::new (),
			};
	}
	
	
	pub fn push_protect (&mut self, _target : &str) -> &mut Self {
		let _target = EntrySelector::if_matches_path (Pattern::glob (_target) .unwrap ());
		self.rules.push (TargetRule::Protect { target : _target });
		self
	}
	
	pub fn push_unlink (&mut self, _target : &str) -> &mut Self {
		let _target = EntrySelector::if_matches_path (Pattern::glob (_target) .unwrap ());
		self.rules.push (TargetRule::Unlink { target : _target });
		self
	}
	
	
	pub fn push_copy (&mut self, _source : &str, _target : &str) -> &mut Self {
		let _source = EntrySelector::if_matches_path (Pattern::glob (_source) .unwrap ());
		self.rules.push (TargetRule::Copy { source : _source.into (), target : _target.into () });
		self
	}
	
	pub fn push_copy_flatten (&mut self, _source : &str, _target : &str) -> &mut Self {
		let _source = EntrySelector::if_matches_path (Pattern::glob (_source) .unwrap ());
		self.rules.push (TargetRule::CopyFlatten { source : _source, target : _target.into () });
		self
	}
	
	
	pub fn push_symlink (&mut self, _source : &str, _target : &str) -> &mut Self {
		let _source = EntrySelector::if_matches_path (Pattern::glob (_source) .unwrap ());
		self.rules.push (TargetRule::Symlink { source : _source.into (), target : _target.into () });
		self
	}
	
	pub fn push_symlink_flatten (&mut self, _source : &str, _target : &str) -> &mut Self {
		let _source = EntrySelector::if_matches_path (Pattern::glob (_source) .unwrap ());
		self.rules.push (TargetRule::SymlinkFlatten { source : _source, target : _target.into () });
		self
	}
	
	
	pub fn push_make_dir (&mut self, _target : &str) -> &mut Self {
		self.rules.push (TargetRule::MakeDir { target : _target.into () });
		self
	}
	
	pub fn push_make_symlink (&mut self, _target : &str, _link : &str) -> &mut Self {
		self.rules.push (TargetRule::MakeSymlink { target : _target.into (), link : _link.into () });
		self
	}
}




impl Entry {
	
	pub fn path_display (&self) -> path::Display<'_> {
		let _path : &Path = self.path.as_ref ();
		return _path.display ();
	}
}


impl TargetDescriptor {
	
	pub fn path_display (&self) -> path::Display<'_> {
		let _path : &Path = self.path.as_ref ();
		return _path.display ();
	}
}

