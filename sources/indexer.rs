

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
		
		let _entry = match build_entry (_root, _entry) {
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
			if _entry.is_dir {
				log_trace! (0xb64e6f82, "dropping `{}`;", _entry.path_0_display ());
				_walker.skip_current_dir ();
			}
		}
		
		if _decision.collect {
			log_trace! (0xc23bcdc0, "including `{}`;", _entry.path_0_display ());
			_collector.push (_entry);
		} else {
			log_trace! (0x08b79f02, "excluding `{}`;", _entry.path_0_display ());
		}
	}
	
	return Ok (());
}




pub fn build_tree (_entries : Vec<fsas::Entry>) -> Outcome<BTreeMap<OsString, fsas::Entry>> {
	
	let mut _tree = BTreeMap::new ();
	
	for _entry in _entries.into_iter () {
		if let Some (_entry) = _tree.insert (_entry.path.clone (), _entry) {
			fail! (0x4314750b, "unexpected duplicate path `{}`", _entry.path_display ());
		}
	}
	
	Ok (_tree)
}




fn build_entry (_root : &Path, _entry : walkdir::DirEntry) -> Outcome<fsas::Entry> {
	
	let _is_dir = _entry.file_type () .is_dir ();
	let _is_symlink = _entry.path_is_symlink ();
	let _depth = _entry.depth ();
	let _path = _entry.into_path ();
	
	let _relative_path = match _path.strip_prefix (_root) {
		Ok (_stripped_path) =>
			if _stripped_path.has_root () {
				OsString::from (_stripped_path)
			} else {
				let _stripped_path = _stripped_path.as_os_str ();
				let mut _relative_path = OsString::with_capacity (1 + _stripped_path.len ());
				_relative_path.push (path::Component::RootDir.as_os_str ());
				_relative_path.push (_stripped_path);
				_relative_path
			}
		Err (_error) =>
			fail! (0x79d3baaa, "invalid stripping for path `{}`", _path.display ()),
	};
	
	let _name : OsString = match _path.file_name () {
		Some (_name) if _name == "" =>
			fail! (0x706787d7, "invalid empty name for path `{}`", _path.display ()),
		Some (_name) if (_name == ".") && (_depth == 0) =>
			".".into (),
		Some (_name) if _name == "." =>
			fail! (0xdd2d6adb, "invalid dot name for path `{}`", _path.display ()),
		Some (_name) =>
			_name.into (),
		None if _depth == 0 =>
			"".into (),
		None =>
			fail! (0xbd9a2b27, "invalid empty name for path `{}`", _path.display ()),
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
	
	let _is_symlink = _kind_symlink.is_symlink ();
	let _is_dir = _kind_follow.is_dir ();
	let _is_file = _kind_follow.is_file ();
	let _is_hidden = (_name.len () > 1) && (_name.as_bytes () [0] == b'.');
	
	let _entry = fsas::Entry {
			path_0 : _path,
			path : _relative_path,
			name : _name,
			depth : _depth,
			kind_symlink : _kind_symlink,
			metadata_symlink : _metadata_symlink,
			kind_follow : _kind_follow,
			metadata_follow : _metadata_follow,
			is_hidden : _is_hidden,
			is_symlink : _is_symlink,
			is_dir : _is_dir,
			is_file : _is_file,
		};
	
	// FIXME:  Sanity checks!
	
	return Ok (_entry);
}

