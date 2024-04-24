//! Testing [camigo_helpers].

// Based on https://mozilla.github.io/application-services/book/design/test-faster.html.
//
// This extra module level is needed here, because [[test]] makes this file the top of the test
// crate (above any modules), rather than make it `tests` module itself.
mod tests {
    mod macros_cami_tests_basic;
    mod macros_cami_tests_party;
    mod macros_core_tests_basic;
}
