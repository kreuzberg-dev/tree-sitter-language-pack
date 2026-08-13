//! Compiles every bundled query (`highlights.scm`, `injections.scm`, `locals.scm`,
//! `tags.scm`, `indents.scm`, `folds.scm`) for every language actually built into
//! this binary, so a grammar bump that silently invalidates a query — vendored or
//! a hand-written `query-overlays/` overlay — fails a test instead of shipping.
//!
//! Requires `TSLP_LANGUAGES` (or the `download` feature with a populated cache) at
//! build time: this crate links zero grammars by default, and a naive sweep over an
//! empty language set would pass vacuously. Run with, for example:
//! `TSLP_LANGUAGES=python,rust,dart,java,kotlin,php,scala,cpp,swift,csharp,ruby cargo test \
//!  -p tree-sitter-language-pack --test query_compilation`

// ~keep Integration tests are separate crates and do not inherit the library's crate-root
// ~keep allow, so the skip notice below needs its own opt-in to the workspace print_stderr deny.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]

use tree_sitter_language_pack::{LanguageRegistry, QueryKind, get_query};

/// Languages with a hand-written `query-overlays/{lang}/tags.scm` that overrides the
/// vendored file (see `crates/ts-pack-core/build.rs::effective_query_path`). A grammar
/// bump cannot regenerate these — they must be hand-fixed, so their compile failures
/// are asserted separately from the bulk sweep. ~keep
const OVERLAY_TAGS_LANGUAGES: [&str; 9] =
    ["dart", "java", "kotlin", "php", "scala", "cpp", "swift", "csharp", "ruby"];

const ALL_QUERY_KINDS: [QueryKind; 6] = [
    QueryKind::Highlights,
    QueryKind::Injections,
    QueryKind::Locals,
    QueryKind::Tags,
    QueryKind::Indents,
    QueryKind::Folds,
];

/// One accumulated failure: which language, which query kind, and the tree-sitter error.
struct QueryFailure {
    language: String,
    kind: QueryKind,
    error: String,
}

impl std::fmt::Display for QueryFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {:?}: {}", self.language, self.kind, self.error)
    }
}

/// Compiles every bundled query kind for `language`, returning the count of queries that
/// actually compiled (kind bundled and `Query::new` succeeded) and appending any compile
/// failure to `failures`. A bundled-but-absent kind (`Ok(None)`) is not a failure.
fn compile_all_kinds_for(language: &str, failures: &mut Vec<QueryFailure>) -> usize {
    let mut compiled = 0usize;
    for kind in ALL_QUERY_KINDS {
        match get_query(language, kind) {
            Ok(Some(_)) => compiled += 1,
            Ok(None) => {}
            Err(error) => failures.push(QueryFailure {
                language: language.to_string(),
                kind,
                error: error.to_string(),
            }),
        }
    }
    compiled
}

#[test]
fn should_compile_every_bundled_query_for_every_language_built_into_this_binary() {
    let registry = LanguageRegistry::new();
    let languages = registry.available_languages();

    // ~keep The single most important assertion here: a build with zero grammars linked
    // ~keep (no TSLP_LANGUAGES, no download cache) makes every loop below a no-op, and a
    // ~keep no-op sweep is indistinguishable from a passing one unless it fails loudly.
    assert!(
        !languages.is_empty(),
        "no languages built into this binary — set TSLP_LANGUAGES (e.g. \
         TSLP_LANGUAGES=python,rust,dart,java,kotlin,php,scala,cpp,swift,csharp,ruby) and rebuild \
         before running this test, otherwise it vacuously passes"
    );

    let mut failures: Vec<QueryFailure> = Vec::new();
    let mut total_compiled = 0usize;
    for language in &languages {
        total_compiled += compile_all_kinds_for(language, &mut failures);
    }

    assert!(
        total_compiled > 0,
        "{} languages were built ({languages:?}) but zero bundled queries compiled — \
         either no .scm files are bundled for any of them, or every one failed to compile \
         (see failures below)\nfailures:\n{}",
        languages.len(),
        failures.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );

    assert!(
        failures.is_empty(),
        "{} bundled quer{} failed to compile:\n{}",
        failures.len(),
        if failures.len() == 1 { "y" } else { "ies" },
        failures.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn should_compile_the_hand_written_tags_overlay_for_every_overlay_language_built_into_this_binary() {
    let registry = LanguageRegistry::new();

    let mut present_overlay_languages: Vec<&str> = Vec::new();
    let mut failures: Vec<QueryFailure> = Vec::new();

    for &language in &OVERLAY_TAGS_LANGUAGES {
        if registry.get_language(language).is_err() {
            // ~keep Not built in this run; matches the skip-per-entry convention used by
            // ~keep intel::tests::has_grammar for partial `TSLP_LANGUAGES` builds.
            continue;
        }
        present_overlay_languages.push(language);

        match get_query(language, QueryKind::Tags) {
            Ok(Some(_)) => {}
            Ok(None) => failures.push(QueryFailure {
                language: language.to_string(),
                kind: QueryKind::Tags,
                error: "expected a hand-written query-overlays/<lang>/tags.scm but none is bundled".to_string(),
            }),
            Err(error) => failures.push(QueryFailure {
                language: language.to_string(),
                kind: QueryKind::Tags,
                error: error.to_string(),
            }),
        }
    }

    if present_overlay_languages.is_empty() {
        eprintln!(
            "SKIPPED should_compile_the_hand_written_tags_overlay_for_every_overlay_language_built_into_this_binary: \
             none of {OVERLAY_TAGS_LANGUAGES:?} are built into this binary — set TSLP_LANGUAGES to include at least \
             one of them to exercise the hand-written overlays"
        );
        return;
    }

    assert!(
        failures.is_empty(),
        "{} of {} built overlay language(s) {present_overlay_languages:?} failed to compile their hand-written \
         tags.scm overlay:\n{}",
        failures.len(),
        present_overlay_languages.len(),
        failures.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
    );
}
