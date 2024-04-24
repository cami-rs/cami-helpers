# Why is this NOT under tests/, but under subtests?

We don't use [Cargo target auto-discovery](https://doc.rust-lang.org/nightly/cargo/reference/cargo-targets.html#target-auto-discovery) for this. Why? Because standard [Integration Tests](https://doc.rust-lang.org/nightly/book/ch11-03-test-organization.html#integration-tests) can't have their own `Cargo.toml`.

We need a separate `Cargo.toml`, so this can depend on both `camigo` and `camigo-helpers`. We can't
use `[dev-dependencies]` to link to `camigo`, because `camigo` itself depends on `camigo-helpers`.
