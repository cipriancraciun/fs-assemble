

use crate::prelude::*;




pub struct Entry {
	pub path : PathBuf,
	pub name : OsString,
	pub depth : usize,
	pub kind_follow : fs::FileType,
	pub kind_symlink : fs::FileType,
	pub metadata_follow : fs::Metadata,
	pub metadata_symlink : fs::Metadata,
}


impl Entry {
	
	pub fn is_symlink (&self) -> bool {
		return self.kind_symlink.is_symlink ();
	}
	
	pub fn is_dir (&self) -> bool {
		return self.kind_follow.is_dir ();
	}
	
	pub fn is_file (&self) -> bool {
		return self.kind_follow.is_file ();
	}
}




pub trait IndexFilter {
	
	fn filter (&self, _entry : &Entry) -> Outcome<IndexDecision>;
}

pub struct IndexDecision {
	pub collect : bool,
	pub recurse : bool,
}




pub struct FilterRules {
	pub rules : Vec<FilterRule>,
	pub skip_hidden : bool,
	pub recurse_symlinks : bool,
}

pub enum FilterRule {
	
	Include {
		selector : Selector,
	},
	Exclude {
		selector : Selector,
	},
}




pub enum TargetRule {
	
	Protect {
		selector : Selector,
	},
	
	Delete {
		selector : Selector,
	},
	
	Mkdir {
		path : String,
	},
	
	CopyFlat {
		source : Selector,
		target : String,
	},
	
	CopyRename {
		source : Selector,
		renaming : Renaming,
	}
}


pub enum Selector {
	
	Exact {
		pattern : String,
		name_only : bool,
	},
	Glob {
		pattern : String,
		name_only : bool,
	},
	Regex {
		pattern : String,
		name_only : bool,
	},
}


pub enum Renaming {
	
	Regex (String, String),
}

