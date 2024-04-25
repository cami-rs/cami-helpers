# Types and generative macros for Camigo

See also [Camigo](https://github.com/peter-kehl/camigo).

## "cami" and "core"

Any `cami_*` macros implement Camigo's traits (TODO LINKS), or, macros generating struct/tuple
wrappers. (You may want to apply other `cami_*` macros to such wrappers, to implement Camigo's
traits).

Any `core_*` macros implement `core` traits ([PartialEq], [PartialOrd], [Ord]) (TODO LINKS) for
types that implement Camigo's traits.

## "core" and no_std

If you haven't noticed `core` top-level module in Rust, think of it as a subset of `std`. (Any
functionality present in both `std` and `core` can be referred through either, as if through an
alias.)

Embedded/WASM/`no_std` developers: Any code from `cami_*` and `core_*` macros works  in `no_std`,
and it doesn't need `alloc`.

## Why is this separate from Camigo?

To keep it more manageable.

`camigo` contains only traits, their implementations and wrappers. As such, it may evolve much
faster than `camigo-helpers`.
