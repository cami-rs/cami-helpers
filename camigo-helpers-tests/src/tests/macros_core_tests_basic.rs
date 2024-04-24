use camigo_helpers::{core_eq, core_ord, core_partial_eq, core_partial_ord, core_wrap_struct};

core_wrap_struct! { StdWrap }
core_wrap_struct! { StdWrap2 <T> T }
core_wrap_struct! { [Clone, Debug] StdWrap3 <T> t T }

core_partial_eq! { StdWrap <T> T }
core_eq! { StdWrap <T> T }
core_partial_ord! { StdWrap <T> T }
core_ord! { StdWrap <T> T }
