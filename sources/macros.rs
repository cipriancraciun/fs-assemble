



macro_rules! fail {
	( $_code : expr, $_message : expr ) => {
		return Err ($crate::tools::error ($_code, format_args! ($_message)));
	};
	( $_code : expr, $_format : expr, $_arguments : expr ) => {
		return Err ($crate::tools::error ($_code, format_args! ($_format, $_arguments)));
	};
}




macro_rules! log_define {
	( $_name : ident, $_slug : literal ) => {
		
		#[ allow (unused_macros) ]
		macro_rules! $_name {
			( $_code : expr, $_message : expr ) => {
				$crate::tools::log ($_slug, $_code, format_args! ($_message));
			};
			( $_code : expr, $_format : expr, $_arguments : tt ) => {
				$crate::tools::log ($_slug, $_code, format_args! ($_format, $_arguments));
			};
		}
	}
}

log_define! (log_error, "[ee]");
log_define! (log_warning, "[ww]");
log_define! (log_notice, "[ii]");
log_define! (log_information, "[ii]");
log_define! (log_debug, "[dd]");

