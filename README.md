# Types and generative macros for Camigo

See also [Camigo](https://github.com/peter-kehl/camigo).

## "cami" and "core"

Any `cami_*` macros implement Camigo's traits (TODO LINKS), or, wrapper-generating macros define structs/tuples
(on which you may want to apply other `cami_*` macros to implement Camigo's traits).

Any `core_*` macros implement `core` traits ([PartialEq], [PartialOrd], [Ord]) (TODO LINKS) for
types that implement Camigo's traits.

## "core"

If you haven't seen `core` top-level module in Rust, think of it as a subset of `std`. (Any
functionality present in both `std` and `core` can be referred through either, as if through an
alias.)

Embedded/WASM/no_std developers: Any `core_*` macros do work in `no_std` and don't need `alloc`.
