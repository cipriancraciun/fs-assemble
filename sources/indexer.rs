

use crate::prelude::*;




pub fn index (_root : &Path, _filter : &impl fsas::IndexFilter, _collector : &mut Vec<fsas::Entry>) -> Outcome<()> {
	
	let mut _walker = walkdir::WalkDir::new (_root)
			.same_file_system (true) // FIXME
			.follow_links (true) // FIXME
			.contents_first (false) // FIXME
			.sort_by (|_left, _right| OsStr::cmp (_left.file_name (), _right.file_name ()))
			.into_iter ();
	
	loop {
		
		let _entry = match _walker.next () {
			Some (Ok (_entry)) =>
				_entry,
			Some (Err (_error)) => {
				log_error! (0xacf7a441, "unexpected error encountered while indexing;  ignoring!  ||  {}", _error);
				continue;
			}
			None =>
				break,
		};
		
		log_trace! (0xa54d097c, "indexing `{}`...", _entry.path () .display ());
		
		let _entry = match build_entry (_entry) {
			Ok (_entry) =>
				_entry,
			Err (_error) => {
				log_error! (0x1b32e9be, "unexpected error encountered while indexing;  ignoring!  ||  {}", _error);
				continue;
			}
		};
		
		let _decision = match _filter.filter (&_entry) {
			Ok (_decision) =>
				_decision,
			Err (_error) => {
				log_error! (0xcab934f9, "unexpected error encountered while indexing;  ignoring!  ||  {}", _error);
				continue;
			}
		};
		
		if ! _decision.recurse {
			if _entry.is_dir () {
				log_debug! (0xb64e6f82, "dropping `{}`;", _entry.path.display ());
				_walker.skip_current_dir ();
			}
		}
		
		if _decision.collect {
			log_debug! (0xc23bcdc0, "including `{}`;", _entry.path.display ());
			_collector.push (_entry);
		} else {
			log_debug! (0x08b79f02, "excluding `{}`;", _entry.path.display ());
		}
	}
	
	return Ok (());
}




fn build_entry (_entry : walkdir::DirEntry) -> Outcome<fsas::Entry> {
	
	let _is_dir = _entry.file_type () .is_dir ();
	let _is_symlink = _entry.path_is_symlink ();
	let _depth = _entry.depth ();
	let _path = _entry.into_path ();
	
	let _name = if let Some (_name) = _path.file_name () {
		_name.to_owned ()
	} else {
		".".into ()
	};
	
	let _metadata_symlink = match fs::symlink_metadata (&_path) {
		Ok (_metadata) =>
			_metadata,
		Err (_error) =>
			fail! (0xe43078c0, "failed `lstat` for path `{}`:  {}", _path.display (), _error),
	};
	
	let _kind_symlink = _metadata_symlink.file_type ();
	
	let _metadata_follow = if _kind_symlink.is_symlink () {
		match fs::metadata (&_path) {
			Ok (_metadata) =>
				_metadata,
			Err (_error) =>
				fail! (0xe43078c0, "failed `stat` for path `{}`:  {}", _path.display (), _error),
		}
	} else {
		_metadata_symlink.clone ()
	};
	
	let _kind_follow = _metadata_follow.file_type ();
	
	let _entry = fsas::Entry {
			path : _path,
			name : _name,
			depth : _depth,
			kind_symlink : _kind_symlink,
			metadata_symlink : _metadata_symlink,
			kind_follow : _kind_follow,
			metadata_follow : _metadata_follow,
		};
	
	// FIXME:  Sanity checks!
	
	return Ok (_entry);
}

