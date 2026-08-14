#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: test/bench code prints by design
//! Regression guard for the typst external scanner's serialization round-trip.
//!
//! `vec_u32_serialize` writes the element-count prefix and `vec_u32_deserialize` reads it back.
//! The two must agree on the prefix width. They briefly did not: patches/typst/ narrowed the
//! written prefix to `uint32_t` (4 bytes) while the reader still took `sizeof self->len`
//! (8 bytes), so deserialization picked up 4 bytes of adjacent buffer as the high half of the
//! count and then memmove'd that many elements off the end of tree-sitter's 1 KiB serialization
//! buffer. It crashed on input as small as `#let x = 1`, at a different address every run.
//!
//! Run with the grammar built in:
//! `TSLP_LANGUAGES=typst,typoscript cargo test -p tree-sitter-language-pack --test typst_serialize_overflow`

use tree_sitter_language_pack::{LanguageRegistry, ProcessConfig, process};

const TYPST_SMOKE_INPUT: &str = "#let x = 1";

/// A second grammar that must parse for a typst failure to mean anything. If the harness itself
/// is broken, this fails too and the typst result below is uninterpretable.
const CONTROL_LANGUAGE: &str = "typoscript";

fn require_language(language: &str) {
    let registry = LanguageRegistry::new();
    let available = registry.available_languages();

    // ~keep This tree builds with ZERO grammars unless TSLP_LANGUAGES is set, and process()
    // ~keep on an unbuilt language fails for that reason rather than the one under test. Without
    // ~keep this guard the whole file is indistinguishable from a passing run.
    assert!(
        available.iter().any(|candidate| candidate == language),
        "`{language}` is not built into this binary, so this test proves nothing about the \
         serialization overflow — rebuild with TSLP_LANGUAGES=typst,typoscript. \
         {} language(s) available.",
        available.len()
    );
}

#[test]
fn control_language_parses_so_a_typst_failure_is_attributable() {
    require_language(CONTROL_LANGUAGE);

    let result = process("page = PAGE\n", &ProcessConfig::new(CONTROL_LANGUAGE));
    assert!(
        result.is_ok(),
        "the control grammar failed, so a typst failure would not be attributable to typst: {result:?}"
    );
}

#[test]
fn typst_survives_the_scanner_serialization_round_trip() {
    require_language("typst");

    let result = process(TYPST_SMOKE_INPUT, &ProcessConfig::new("typst"));
    let result = result.unwrap_or_else(|error| panic!("typst failed to process {TYPST_SMOKE_INPUT:?}: {error:?}"));

    assert_eq!(
        result.metrics.error_count, 0,
        "typst parsed {TYPST_SMOKE_INPUT:?} with parse errors: {result:?}"
    );
}
