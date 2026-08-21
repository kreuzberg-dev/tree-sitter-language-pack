#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: test/bench code prints by design
#![allow(clippy::unwrap_used, clippy::expect_used)] // ~keep: a failed setup step in a test must abort loudly
//! Regression guard for `vec_u32_pop` in the typst external scanner.
//!
//! Upstream reads `self->vec[self->len--]`: the post-decrement returns `vec[len]`, one past
//! the last element, and only then shortens the vector. `patches/typst/vec-u32-pop-off-by-one.patch`
//! makes it `vec[--self->len]`.
//!
//! Nothing observable at the grammar's C ABI changes, which is why the defect survived: the
//! length bookkeeping was already correct and every caller in the scanner discards the value.
//! What is left is a four-byte heap over-read, and reads past the end of an allocation are
//! usually harmless in practice — they only fault when they cross into an unmapped page. A
//! test that waits for a signal would pass by luck here, so this file compiles the patched
//! `scanner.c` into a probe library (`typst_vec_u32_pop_probe.c`) and asserts on the value
//! `vec_u32_pop` actually returns.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Written into the slot immediately past the vector's last element before each pop. A pop
/// that reads one past the end returns this instead of the element.
const SENTINEL: u32 = 0xDEAD_BEEF;

/// Pushed in order, so a correct pop sequence returns them reversed.
const PUSHED: [u32; 3] = [7, 8, 9];

/// The scanner sources are laid down by the build script, either in the workspace or in the
/// `OUT_DIR` cache it falls back to. Both are searched, in that order.
fn scanner_source_dir() -> PathBuf {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(root) = project_root() {
        candidates.push(root.join("parsers/typst/src"));
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(profile_dir) = exe.parent().and_then(Path::parent)
        && let Ok(entries) = std::fs::read_dir(profile_dir.join("build"))
    {
        for entry in entries.filter_map(Result::ok) {
            let cache = entry.path().join("out/_parsers");
            candidates.push(cache.join("typst/src"));
            candidates.push(cache.join("parsers/typst/src"));
        }
    }

    candidates
        .iter()
        .find(|dir| dir.join("scanner.c").is_file())
        .cloned()
        .unwrap_or_else(|| {
            // ~keep A skip here would be a false pass: the whole point of this file is that the
            // ~keep defect is invisible unless this exact source is compiled and run.
            panic!(
                "no typst scanner.c found. This test reads the patched grammar source, which a \
                 build materialises; run `cargo build` (optionally with TSLP_LANGUAGES=typst) \
                 first. Looked in: {}",
                candidates
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

/// Walk up to the workspace root, identified the same way the build script identifies it.
fn project_root() -> Option<PathBuf> {
    if let Ok(root) = std::env::var("PROJECT_ROOT") {
        return Some(PathBuf::from(root));
    }
    let mut dir: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("sources/language_definitions.json").exists() && dir.join("patches").is_dir() {
            return Some(dir.to_path_buf());
        }
        dir = dir.parent()?;
    }
}

fn probe_library_name() -> &'static str {
    if cfg!(target_os = "macos") {
        "libtypst_vec_u32_probe.dylib"
    } else {
        "libtypst_vec_u32_probe.so"
    }
}

/// Compile `typst_vec_u32_pop_probe.c` — and, through its `#include`, the patched
/// `scanner.c` — into a shared library in a fresh directory.
fn build_probe(directory: &Path) -> PathBuf {
    let source_dir = scanner_source_dir();
    let scanner = source_dir.join("scanner.c");
    let probe = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/typst_vec_u32_pop_probe.c");
    let output = directory.join(probe_library_name());
    let compiler = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());

    let status = Command::new(&compiler)
        .args(["-std=c11", "-O1", "-g", "-Wno-unused-function", "-shared", "-fPIC"])
        .arg("-I")
        .arg(&source_dir)
        .arg(format!("-DSCANNER_UNDER_TEST=\"{}\"", scanner.display()))
        .arg("-o")
        .arg(&output)
        .arg(&probe)
        .status()
        .unwrap_or_else(|error| panic!("failed to run the C compiler `{compiler}`: {error}"));

    assert!(
        status.success(),
        "`{compiler}` failed to build the probe from {}",
        scanner.display()
    );
    output
}

type PopProbe = unsafe extern "C" fn(u32, *const u32, *mut u32, *mut usize) -> i32;
type EmptyPopProbe = unsafe extern "C" fn(u32, *mut u32, *mut usize) -> i32;

const PROBE_OK: i32 = 0;

/// Three pops of a three-element vector: the values returned and the length after each.
fn run_pop_probe(library: &libloading::Library) -> ([u32; 3], [usize; 3]) {
    let mut values = [0u32; 3];
    let mut lengths = [0usize; 3];
    // ~keep SAFETY: the symbol is defined in typst_vec_u32_pop_probe.c with this signature,
    // ~keep and both out-parameters point at three-element arrays, which is what it writes.
    let outcome = unsafe {
        let probe = library
            .get::<PopProbe>(b"tslp_vec_u32_pop_probe")
            .expect("probe symbol");
        probe(SENTINEL, PUSHED.as_ptr(), values.as_mut_ptr(), lengths.as_mut_ptr())
    };
    assert_eq!(
        outcome, PROBE_OK,
        "the probe could not set up a vector whose slot past the end is inside the allocation, \
         so it never exercised the read under test"
    );
    (values, lengths)
}

#[test]
fn pop_returns_the_last_element_and_never_the_slot_past_it() {
    let directory = tempfile::tempdir().expect("temp dir");
    let library_path = build_probe(directory.path());
    // ~keep SAFETY: the library was produced by the compiler invocation directly above.
    let library = unsafe { libloading::Library::new(&library_path) }
        .unwrap_or_else(|error| panic!("failed to load {}: {error}", library_path.display()));

    let (values, _) = run_pop_probe(&library);

    let expected = [PUSHED[2], PUSHED[1], PUSHED[0]];
    assert_eq!(
        values, expected,
        "vec_u32_pop returned {values:08x?} for a vector pushed as {PUSHED:?}. \
         {SENTINEL:#010x} is the value planted in vec[len], one past the last element, so a pop \
         reporting it read past the end and returned whatever sat there"
    );
}

/// The length must still shrink by one per pop.
///
/// The bug was in the index, not the bookkeeping: `vec[len--]` already left the right length
/// behind. Asserting the lengths keeps a "fix" that moves the decrement out of the expression
/// and forgets to reapply it from passing. ~keep
#[test]
fn each_pop_shortens_the_vector_by_exactly_one() {
    let directory = tempfile::tempdir().expect("temp dir");
    let library_path = build_probe(directory.path());
    // ~keep SAFETY: the library was produced by the compiler invocation directly above.
    let library = unsafe { libloading::Library::new(&library_path) }
        .unwrap_or_else(|error| panic!("failed to load {}: {error}", library_path.display()));

    let (_, lengths) = run_pop_probe(&library);

    assert_eq!(
        lengths,
        [2, 1, 0],
        "three pops of a three-element vector left lengths {lengths:?}"
    );
}

/// Popping an empty vector must not wrap the length.
///
/// No caller does this today — all four check the length first — but the guard those checks
/// rely on inside `vec_u32_pop` is an `assert()` that this file `#define`s to `while (false);`,
/// so it enforces nothing. With the index pre-decremented, an unguarded empty pop would take
/// the length to `SIZE_MAX` and index `vec[SIZE_MAX]`, which is far worse than the four-byte
/// over-read the patch removes. The vector's buffer is still allocated here, so neither form
/// faults and the difference is visible as a value. ~keep
#[test]
fn popping_an_empty_vector_leaves_the_length_at_zero() {
    let directory = tempfile::tempdir().expect("temp dir");
    let library_path = build_probe(directory.path());
    // ~keep SAFETY: the library was produced by the compiler invocation directly above.
    let library = unsafe { libloading::Library::new(&library_path) }
        .unwrap_or_else(|error| panic!("failed to load {}: {error}", library_path.display()));

    let mut value = 0u32;
    let mut length = 0usize;
    // ~keep SAFETY: the symbol is defined in typst_vec_u32_pop_probe.c with this signature.
    let outcome = unsafe {
        let probe = library
            .get::<EmptyPopProbe>(b"tslp_vec_u32_pop_empty_probe")
            .expect("empty probe symbol");
        probe(PUSHED[0], &raw mut value, &raw mut length)
    };
    assert_eq!(
        outcome, PROBE_OK,
        "the probe could not empty the vector without freeing it"
    );

    assert_eq!(
        length, 0,
        "a pop from an empty vector left length {length}; SIZE_MAX means the length wrapped and \
         the read was at vec[SIZE_MAX]"
    );
    assert_eq!(
        value, 0,
        "a pop from an empty vector must report nothing, not a stale element"
    );
}
