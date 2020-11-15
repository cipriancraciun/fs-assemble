

use crate::prelude::*;
use crate::rules::*;




peg::parser! {
	
	pub grammar grammar () for str {
		
		
		
		
		pub rule statement () -> Statement
			= (
				statement_copy_simple_1() /
				statement_copy_simple_2() /
				statement_copy() /
				statement_symlink_simple_1() /
				statement_symlink_simple_2() /
				statement_symlink() /
				statement_make_dir() /
				statement_make_symlink() /
				statement_protect() /
				statement_unlink()
			)
		
		
		pub rule statement_copy_simple_1 () -> Statement
			= "copy" space() _target:path() statement_end()
			{ Statement::TargetRule (TargetRule::Copy { source : EntrySelector::if_matches_path_exact (&_target), target : _target.into () }) }
		
		pub rule statement_copy_simple_2 () -> Statement
			= "copy" space() "to" space() _target:path() space() "from" space() _source:path() statement_end()
			{ Statement::TargetRule (TargetRule::Copy { source : EntrySelector::if_matches_path_exact (&_source), target : _target.into () }) }
		
		pub rule statement_copy () -> Statement
			= "copy" space() "to" space() _target:path() space() _source:selector() statement_end()
			{ Statement::TargetRule (TargetRule::Copy { source : _source, target : _target.into () }) }
		
		
		pub rule statement_symlink_simple_1 () -> Statement
			= "symlink" space () _target:path() statement_end()
			{ Statement::TargetRule (TargetRule::Symlink { source : EntrySelector::if_matches_path_exact (&_target), target : _target.into () }) }
		
		pub rule statement_symlink_simple_2 () -> Statement
			= "symlink" space() "to" space() _target:path() space() "from" space() _source:path() statement_end()
			{ Statement::TargetRule (TargetRule::Symlink { source : EntrySelector::if_matches_path_exact (&_source), target : _target.into () }) }
		
		pub rule statement_symlink () -> Statement
			= "symlink" space() "to" space() _target:path() space() _source:selector() statement_end()
			{ Statement::TargetRule (TargetRule::Symlink { source : _source, target : _target.into () }) }
		
		
		pub rule statement_make_dir () -> Statement
			= "mkdir" space() _target:path() statement_end()
			{ Statement::TargetRule (TargetRule::MakeDir { target : _target.into () }) }
		
		pub rule statement_make_symlink () -> Statement
			= "symlink" space() "to" space() _target:path() space() "external" _link:path() statement_end()
			{ Statement::TargetRule (TargetRule::MakeSymlink { target : _target.into (), link : _link.into () }) }
		
		
		pub rule statement_protect () -> Statement
			= "protect" space() _target:selector() statement_end()
			{ Statement::TargetRule (TargetRule::Protect { target : _target.into () }) }
		
		pub rule statement_unlink () -> Statement
			= "unlink" space() _target:selector() statement_end()
			{ Statement::TargetRule (TargetRule::Unlink { target : _target.into () }) }
		
		
		rule statement_end () -> ()
			= space()? ";"
			{ () }
		
		
		
		
		pub rule script () -> Script
			= space()? _statements:statement()**( space()? ) space()?
			{
				Script {
						statements : _statements,
					}
			}
		
		
		
		
		pub rule selector () -> EntrySelector
			= _matcher:matcher()
			{ EntrySelector::Matches (_matcher) }
		
		
		
		
		pub rule matcher () -> EntryMatcher
			= (
				matcher_from() /
				matcher_glob()
			)
		
		pub rule matcher_from () -> EntryMatcher
			= "exact" space() _path:path()
			{ EntryMatcher::Path (Pattern::exact (&_path)) }
		
		pub rule matcher_glob () -> EntryMatcher
			= "glob" space() _pattern:path()
			{ EntryMatcher::Path (Pattern::glob (&_pattern) .unwrap ()) }
		
		
		
		
		pub rule path () -> String
			= string()
		
		
		pub rule identifier () -> String
			= _span:$( ( letter() / digit() / ['_'] )+ )
			{ _span.into () }
		
		
		pub rule string () -> String
			= string_quoted()
		
		pub rule string_quoted () -> String
			=
				"'"
				_span:$( (
					( !['\\'|'\''] [_] )
					/ ( ['\\'] ['\\'] )
					/ ( ['\\'] ['\''] )
				)* )
				"'"
			{
				_span
					.replace ("\\'", "'")
					.replace ("\\\\", "\\")
			}
		
		
		pub rule letter () -> char
			= _span:$( ['a'..='z'|'A'..='Z'] )
			{ _span.chars () .next () .unwrap () }
		
		pub rule digit () -> char
			= _span:$( ['0'..='9'] )
			{ _span.chars () .next () .unwrap () }
		
		
		pub rule space () -> ()
			= quiet!{ ( [' '|'\t'] / newline() )+ / ( [' '|'\t']+ ( ['\\'] newline() )+ [' '|'\t']* )+ }
			{ () }
		
		pub rule newline () -> ()
			= quiet!{ ['\n'] / ( ['\r'] ['\n'] ) }
			{ () }
	}
}




#[ derive (Clone) ]
#[ derive (Debug) ]
pub enum Statement {
	SourceIndexOption (IndexOption),
	SourceIndexRule (IndexRule),
	TargetIndexOption (IndexOption),
	TargetIndexRule (IndexRule),
	TargetRule (TargetRule),
}




#[ derive (Clone) ]
#[ derive (Debug) ]
pub struct Script {
	pub statements : Vec<Statement>,
}




pub fn parse (_path : &Path) -> Outcome<Script> {
	
	let _data = match fs::read (_path) {
		Ok (_data) =>
			_data,
		Err (_error) =>
			fail! (0x4ca9d919, "failed loading script `{}`:  {}", _path.display (), _error),
	};
	
	let _data = match String::from_utf8 (_data) {
		Ok (_data) =>
			_data,
		Err (_error) =>
			fail! (0xfbccd3b6, "failed loading script `{}`:  {}", _path.display (), _error),
	};
	
	let _script = match grammar::script (&_data) {
		Ok (_script) =>
			_script,
		Err (_error) =>
			fail! (0x2508f535, "failed parsing script `{}`:  {}", _path.display (), _error),
	};
	
	return Ok (_script);
}

