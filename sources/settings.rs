

#[ cfg (debug_assertions) ]
pub const DUMP_VERBOSE : bool = true;

#[ cfg (not (debug_assertions)) ]
pub const DUMP_VERBOSE : bool = false;


pub const EXECUTION_ALLOWED : bool = true;


pub const DUMP_UNIMPORTANT : bool = false;


pub const DUMP_SCRIPT : bool = DUMP_VERBOSE || false;

pub const DUMP_SOURCES_UNHANDLED : bool = DUMP_VERBOSE && DUMP_UNIMPORTANT || false;
pub const DUMP_TARGETS_UNHANDLED : bool = DUMP_VERBOSE && DUMP_UNIMPORTANT || false;

pub const DUMP_DESCRIPTORS_ALL : bool = DUMP_VERBOSE || false;
pub const DUMP_DESCRIPTORS_PLANNING : bool = DUMP_DESCRIPTORS_ALL && DUMP_UNIMPORTANT || false;
pub const DUMP_DESCRIPTORS_PLANNED : bool = DUMP_DESCRIPTORS_ALL && DUMP_UNIMPORTANT || false;
pub const DUMP_DESCRIPTORS_REQUIRED : bool = DUMP_DESCRIPTORS_ALL || false;
pub const DUMP_DESCRIPTORS_SUCCEEDED : bool = DUMP_DESCRIPTORS_ALL && DUMP_UNIMPORTANT || false;
pub const DUMP_DESCRIPTORS_FAILED : bool = DUMP_DESCRIPTORS_ALL || false;
pub const DUMP_DESCRIPTORS_SKIPPED : bool = DUMP_DESCRIPTORS_ALL && DUMP_UNIMPORTANT || false;


pub const DUMP_LOG_VERBOSE : bool = DUMP_VERBOSE || false;
pub const DUMP_LOG_LEVEL : u16 = if DUMP_LOG_VERBOSE { LOG_LEVEL_DEBUG } else { LOG_LEVEL_NOTICE };
pub const DUMP_LOG_CUT : bool = DUMP_LOG_VERBOSE || (DUMP_LOG_LEVEL <= LOG_LEVEL_DEBUG) || false;

pub const LOG_LEVEL_ERROR : u16 = 60_000;
pub const LOG_LEVEL_WARNING : u16 = 50_000;
pub const LOG_LEVEL_NOTICE : u16 = 40_000;
pub const LOG_LEVEL_INFORMATION : u16 = 30_000;
pub const LOG_LEVEL_DEBUG : u16 = 20_000;
pub const LOG_LEVEL_TRACE : u16 = 10_000;

