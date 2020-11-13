

use crate::prelude::*;




#[ derive (Clone) ]
#[ derive (Debug) ]
pub struct Entry {
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
	pub link : Option<OsString>,
}




pub trait IndexFilter {
	
	fn filter (&self, _entry : &Entry) -> Outcome<IndexDecision>;
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub struct IndexDecision {
	pub collect : bool,
	pub recurse : bool,
}




#[ derive (Clone) ]
#[ derive (Debug) ]
pub struct IndexRules {
	pub rules : Vec<IndexRule>,
	pub symlinks_collect : bool,
	pub symlinks_recurse : bool,
	pub hidden_collect : bool,
	pub hidden_recurse : bool,
	pub fallback_collect : bool,
	pub fallback_recurse : bool,
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum IndexRule {
	
	Include {
		selector : EntrySelector,
	},
	Exclude {
		selector : EntrySelector,
	},
}




#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum EntrySelector {
	Always,
	Never,
	Matches (EntryMatcher),
	NotMatches (EntryMatcher),
	Any (Vec<EntrySelector>),
	All (Vec<EntrySelector>),
	None (Vec<EntrySelector>),
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum EntryMatcher {
	
	Path (Pattern),
	Name (Pattern),
	
	IsSymlink,
	IsDir,
	IsFile,
	IsHidden,
	
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum Pattern {
	Exact (OsString),
	Prefix (OsString),
	Suffix (OsString),
	Glob (globset::GlobSet),
	Regex (regexb::RegexSet),
}




#[ derive (Clone) ]
#[ derive (Debug) ]
pub struct TargetRules {
	pub rules : Vec<TargetRule>,
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum TargetRule {
	
	Protect {
		target : EntrySelector,
	},
	
	Unlink {
		target : EntrySelector,
	},
	
	Copy {
		source : EntrySelector,
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
	
	Symlink {
		source : EntrySelector,
		target : OsString,
	},
	
	SymlinkFlatten {
		source : EntrySelector,
		target : OsString,
	},
	
	SymlinkRename {
		source : EntrySelector,
		renaming : Renaming,
	},
	
	MakeDir {
		target : OsString,
	},
	
	MakeSymlink {
		target : OsString,
		link : OsString,
	},
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum Renaming {
	
	Regex {
		pattern : regexb::Regex,
		replacement : OsString,
	},
}




#[ derive (Clone) ]
#[ derive (Debug) ]
pub struct TargetDescriptor {
	pub path : OsString,
	pub operation : TargetOperation,
}


#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum TargetOperation {
	
	Protect {
		existing : Entry,
	},
	
	Unlink {
		existing : Entry,
	},
	
	Copy {
		source : Entry,
		existing : Option<Entry>,
	},
	
	Symlink {
		source : Entry,
		existing : Option<Entry>,
	},
	
	MakeDir {
		existing : Option<Entry>,
	},
	
	MakeSymlink {
		link : OsString,
		existing : Option<Entry>,
	},
}

