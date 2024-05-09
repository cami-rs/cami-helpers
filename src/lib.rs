// @TODO move to README.md?

//! Contrary to Rust macro hygiene, most macros from here require [cami] crate name not to be
//! "shadowed" by any other Rust symbol (that would have the same name). (See source code for
//! reasons.)
//!
// These macros can't use a universal path `::cami::...`. Why? Because these macros are also used in
// [cami] crate itself. No crate can access itself under its own name (so `::cami` doesn't work
// within [cami] crate, unfortunately).
//
// Those macros COULD
// 1. have an extra optional parameter, which would be a path to [cami] crate itself,
// 2. the macros could default such a parameter to `::cami`, and
// 3. special: any use of the macros from within [cami] crate itself would provide a path based on
//    an `pub use crate as cami;` (as this crate already does).
//
// BUT, we would have to duplice all matcher parts/patterns of those macros - to match with this
// optional parameter, and without it. AND, the matcher parts/patterns themselves are already LONG.

#![cfg_attr(not(feature = "std"), no_std)]

pub use locality::{
    debug_fail_unreachable_for_local, debug_fail_unreachable_for_non_local, Locality,
};

mod locality;

#[macro_use]
mod macros_cami;
pub use macros_cami::always_equal_ref;
pub mod prelude;

#[cfg(feature = "alloc")]
extern crate alloc;
