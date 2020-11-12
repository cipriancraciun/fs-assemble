

use crate::prelude::*;




pub struct Entry {
	pub path_0 : PathBuf,
	pub path : OsString,
	pub name : OsString,
	pub depth : usize,
	pub kind_follow : fs::FileType,
	pub kind_symlink : fs::FileType,
	pub metadata_follow : fs::Metadata,
	pub metadata_symlink : fs::Metadata,
	pub is_symlink : bool,
	pub is_file : bool,
	pub is_dir : bool,
	pub is_hidden : bool,
}




pub trait IndexFilter {
	
	fn filter (&self, _entry : &Entry) -> Outcome<IndexDecision>;
}

pub struct IndexDecision {
	pub collect : bool,
	pub recurse : bool,
}




pub struct IndexRules {
	pub rules : Vec<IndexRule>,
	pub symlinks_collect : bool,
	pub symlinks_recurse : bool,
	pub hidden_collect : bool,
	pub hidden_recurse : bool,
}

pub enum IndexRule {
	
	Include {
		selector : EntrySelector,
	},
	Exclude {
		selector : EntrySelector,
	},
}




pub enum EntrySelector {
	Always,
	Never,
	Matches (EntryMatcher),
	NotMatches (EntryMatcher),
	Any (Vec<EntrySelector>),
	All (Vec<EntrySelector>),
	None (Vec<EntrySelector>),
}


pub enum EntryMatcher {
	
	Path (Pattern),
	Name (Pattern),
	
	IsSymlink,
	IsDir,
	IsFile,
	IsHidden,
	
}


pub enum Pattern {
	Exact (OsString),
	Prefix (OsString),
	Suffix (OsString),
	Glob (globset::GlobSet),
	Regex (regexb::RegexSet),
}




pub enum TargetRule {
	
	Protect {
		selector : EntrySelector,
	},
	
	Delete {
		selector : EntrySelector,
	},
	
	MkDir {
		path : OsString,
	},
	
	Copy {
		source : OsString,
		target : OsString,
	},
	
	CopyFlatten {
		source : EntrySelector,
		target : OsString,
	},
	
	CopyRename {
		source : EntrySelector,
		renaming : Renaming,
	},
}


pub enum Renaming {
	
	Regex {
		pattern : regexb::Regex,
		replacement : OsString,
	},
}

