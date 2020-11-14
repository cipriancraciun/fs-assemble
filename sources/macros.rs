



macro_rules! fail {
	( $_code : expr, $_message : expr ) => {
		return Err ($crate::tools::error ($_code, format_args! ($_message)));
	};
	( $_code : expr, $_format : expr, $( $_arguments : expr ),* ) => {
		return Err ($crate::tools::error ($_code, format_args! ($_format, $( $_arguments ),* )));
	};
}

macro_rules! fail_unimplemented {
	( $_code : expr ) => {
		return Err ($crate::tools::error ($_code, format_args! ("not implemented")));
	};
}




macro_rules! log_define {
	( $_name : ident, $_slug : literal, $_level : literal ) => {
		
		#[ allow (unused_macros) ]
		macro_rules! $_name {
			( $_code : expr, $_message : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, format_args! ($_message));
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, format_args! ($_format, $_argument_1));
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, format_args! ($_format, $_argument_1, $_argument_2));
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr, $_argument_3 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, format_args! ($_format, $_argument_1, $_argument_2, $_argument_3));
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr, $_argument_3 : expr, $_argument_4 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, format_args! ($_format, $_argument_1, $_argument_2, $_argument_3, $_argument_4));
			};
			( $_code : expr, $_format : expr, $_argument_1 : expr, $_argument_2 : expr, $_argument_3 : expr, $_argument_4 : expr, $_argument_5 : expr ) => {
				$crate::tools::log ($_slug, $_level, $_code, format_args! ($_format, $_argument_1, $_argument_2, $_argument_3, $_argument_4, $_argument_5));
			};
		}
	}
}

log_define! (log_error, "[ee]", 60000);
log_define! (log_warning, "[ww]", 50000);
log_define! (log_notice, "[ii]", 40000);
log_define! (log_information, "[ii]", 30000);
log_define! (log_debug, "[dd]", 20000);
log_define! (log_trace, "[dd]", 10000);

#[ allow (unused_macros) ]
macro_rules! log_cut {
	() => {
		$crate::tools::log_cut ();
	};
}

