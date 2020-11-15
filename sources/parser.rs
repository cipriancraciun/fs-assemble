

use crate::prelude::*;
use crate::rules::*;




peg::parser! {
	
	pub grammar grammar () for str {
		
		
		
		
		pub rule statement () -> Statement
			= (
				statement_copy_simple_1() /
				statement_copy_simple_2() /
				statement_copy_find() /
				statement_symlink_simple_1() /
				statement_symlink_simple_2() /
				statement_symlink_find() /
				statement_make_dir() /
				statement_make_symlink() /
				statement_protect() /
				statement_unlink()
			)
		
		
		pub rule statement_copy_simple_1 () -> Statement
			= "copy" ws() _target:path() statement_end()
			{ Statement::TargetRule (TargetRule::Copy { source : EntrySelector::if_matches_path_exact (&_target), target : _target.into () }) }
		
		pub rule statement_copy_simple_2 () -> Statement
			= "copy" ws() "to" ws() _target:path() ws() "from" ws() _source:path() statement_end()
			{ Statement::TargetRule (TargetRule::Copy { source : EntrySelector::if_matches_path_exact (&_source), target : _target.into () }) }
		
		pub rule statement_copy_find () -> Statement
			= "copy" ws() "to" ws() _target:path() ws() "find" ws() _source:selector() statement_end()
			{ Statement::TargetRule (TargetRule::CopyFlatten { source : _source, target : _target.into () }) }
		
		
		pub rule statement_symlink_simple_1 () -> Statement
			= "symlink" space () _target:path() statement_end()
			{ Statement::TargetRule (TargetRule::Symlink { source : EntrySelector::if_matches_path_exact (&_target), target : _target.into () }) }
		
		pub rule statement_symlink_simple_2 () -> Statement
			= "symlink" ws() "to" ws() _target:path() ws() "from" ws() _source:path() statement_end()
			{ Statement::TargetRule (TargetRule::Symlink { source : EntrySelector::if_matches_path_exact (&_source), target : _target.into () }) }
		
		pub rule statement_symlink_find () -> Statement
			= "symlink" ws() "to" ws() _target:path() ws() "find" _source:selector() statement_end()
			{ Statement::TargetRule (TargetRule::SymlinkFlatten { source : _source, target : _target.into () }) }
		
		
		pub rule statement_make_dir () -> Statement
			= "mkdir" ws() _target:path() statement_end()
			{ Statement::TargetRule (TargetRule::MakeDir { target : _target.into () }) }
		
		pub rule statement_make_symlink () -> Statement
			= "symlink" ws() "to" ws() _target:path() ws() "external" _link:path() statement_end()
			{ Statement::TargetRule (TargetRule::MakeSymlink { target : _target.into (), link : _link.into () }) }
		
		
		pub rule statement_protect () -> Statement
			= "protect" ws() _target:selector() statement_end()
			{ Statement::TargetRule (TargetRule::Protect { target : _target.into () }) }
		
		pub rule statement_unlink () -> Statement
			= "unlink" ws() _target:selector() statement_end()
			{ Statement::TargetRule (TargetRule::Unlink { target : _target.into () }) }
		
		
		rule statement_end () -> ()
			= ws()? ";"
			{ () }
		
		
		
		
		pub rule script () -> Script
			= ws()? _statements:statement()**( ws()? ) ws()?
			{ Script { statements : _statements } }
		
		
		
		
		pub rule selector () -> EntrySelector
			= (
				selector_negate() /
				selector_matcher() /
				selector_when_all() /
				selector_when_any() /
				selector_when_none() /
				"always" { EntrySelector::Always } /
				"never" { EntrySelector::Never }
			)
		
		pub rule selector_matcher () -> EntrySelector
			= _matcher:matcher()
			{ EntrySelector::Matches (_matcher) }
		
		pub rule selector_negate () -> EntrySelector
			= "not" ws() _selector:selector()
			{ _selector.negate () }
		
		pub rule selector_when_all () -> EntrySelector
			= "when" ws() "all" ws()? "{" ws()? _selectors:selector()**( ws()? "," ws()? ) ws()? "}"
			{ EntrySelector::All (_selectors) }
		
		pub rule selector_when_any () -> EntrySelector
			= "when" ws() "any" ws()? "{" ws()? _selectors:selector()**( ws()? "," ws()? ) ws()? "}"
			{ EntrySelector::Any (_selectors) }
		
		pub rule selector_when_none () -> EntrySelector
			= "when" ws() "none" ws()? "{" ws()? _selectors:selector()**( ws()? "," ws()? ) ws()? "}"
			{ EntrySelector::None (_selectors) }
		
		
		
		
		pub rule matcher () -> EntryMatcher
			= (
				matcher_path() /
				matcher_name() /
				matcher_kind() /
				matcher_simple()
			)
		
		pub rule matcher_path () -> EntryMatcher
			= "path" ws() _pattern:pattern()
			{ EntryMatcher::Path (_pattern) }
		
		pub rule matcher_name () -> EntryMatcher
			= "name" ws() _pattern:pattern()
			{ EntryMatcher::Name (_pattern) }
		
		pub rule matcher_kind () -> EntryMatcher
			= (
				
				( "is" ws() ( "folder" / "directory" / "dir" ) ) { EntryMatcher::IsDir } /
				( "is" ws() ( "file" / "regular" ) ) { EntryMatcher::IsFile } /
				( "is" ws() "symlink" ) { EntryMatcher::IsSymlink } /
				( "is" ws() "hidden" ) { EntryMatcher::IsHidden } /
				
				( "is" ws() "not" ws() ( "folder" / "directory" / "dir" ) ) { EntryMatcher::IsNotDir } /
				( "is" ws() "not" ws() ( "file" / "regular" ) ) { EntryMatcher::IsNotFile } /
				( "is" ws() "not" ws() "symlink" ) { EntryMatcher::IsNotSymlink } /
				( "is" ws() "not" ws() "hidden" ) { EntryMatcher::IsNotHidden }
			)
		
		pub rule matcher_simple () -> EntryMatcher
			= (
				_pattern:pattern() { EntryMatcher::Path (_pattern) } /
				_path:path() { EntryMatcher::Path (Pattern::exact (&_path)) }
			)
		
		
		
		
		pub rule pattern () -> Pattern
			= (
				pattern_exact () /
				pattern_prefix () /
				pattern_suffix () /
				pattern_glob () /
				pattern_regex ()
			)
		
		pub rule pattern_exact () -> Pattern
			= "exact" ws() _path:path()
			{ Pattern::exact (&_path) }
		
		pub rule pattern_prefix () -> Pattern
			= "prefix" ws() _path:path()
			{ Pattern::prefix (&_path) }
		
		pub rule pattern_suffix () -> Pattern
			= "suffix" ws() _path:path()
			{ Pattern::suffix (&_path) }
		
		pub rule pattern_glob () -> Pattern
			= "glob" ws() _pattern:path()
			{ Pattern::glob (&_pattern) .unwrap () }
		
		pub rule pattern_regex () -> Pattern
			= "regex" ws() _pattern:path()
			{ Pattern::regex (&_pattern) .unwrap () }
		
		
		
		
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
			= quiet!{ [' '|'\t']+ / ( [' '|'\t']+ ( ['\\'] newline() )+ [' '|'\t']* )+ }
			{ () }
		
		pub rule newline () -> ()
			= quiet!{ ['\n'] / ( ['\r'] ['\n'] ) }
			{ () }
		
		pub rule comment () -> ()
			= quiet!{ ( ['#'] ( !newline() [_] )* newline() )+ }
			{ () }
		
		pub rule ws () -> ()
			= quiet!{
				(
					space() comment() /
					space() /
					newline() comment() /
					newline()
				)+
			}
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

