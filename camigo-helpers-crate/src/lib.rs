// @TODO move to README.md?

//! Contrary to Rust macro hygiene, most macros from here require [camigo] crate name not to be
//! "shadowed" by any other Rust symbol (that would have the same name). (See source code for
//! reasons.)
//!
// These macros can't use a universal path `::camigo::...`. Why? Because these macros are also used
// in [camigo] crate itself. No crate can access itself under its own name (so `::camigo` doesn't
// work within [camigo] crate, unfortunately).
//
// Those macros COULD
// 1. have an extra optional parameter, which would be a path to [camigo] crate itself,
// 2. the macros could default such a parameter to `::camigo`, and
// 3. special: any use of the macros from within [camigo] crate itself would provide a path based on
//    an `pub use crate as camigo;` (as this crate already does).
//
// BUT, we would have to duplice all matcher parts/patterns of those macros - to match with this
// optional parameter, and without it. AND, the matcher parts/patterns are already LONG.

#![cfg_attr(not(feature = "std"), no_std)]

pub use locality::{
    debug_fail_unreachable_for_local, debug_fail_unreachable_for_non_local, Locality,
};

mod locality;

#[macro_use]
mod macros_cami;
pub use macros_cami::always_equal_ref;
#[macro_use]
mod macros_core;
pub mod prelude;

#[cfg(feature = "alloc")]
extern crate alloc;
