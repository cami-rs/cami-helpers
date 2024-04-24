//! Most macros require [camigo] crate not to be "shadowed" by any other symbol with that name.
//! (Contrary to Rust macro hygiene, these macros can't use `::camigo`. Why? Because these macros
//! are also used in [camigo] crate itself, and this crate itself can't access `::camigo`,
//! unfortunately.)

pub use locality::{
    debug_fail_unreachable_for_local, debug_fail_unreachable_for_non_local, Locality,
};

mod locality;

#[macro_use]
mod macros_ca;
pub use macros_ca::always_equal_ref;
pub mod prelude;
