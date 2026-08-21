#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: test/bench code prints by design
#![allow(clippy::unwrap_used, clippy::expect_used)] // ~keep: a failed setup step in a test must abort loudly
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

// ---------------------------------------------------------------------------
// Direct external-scanner ABI tests.
//
// The tests above can only observe this bug if it happens to fault, and whether it
// faults depends on what follows the serialization buffer in memory -- it reproduces
// on linux-x86_64 and not on macOS/arm64. That makes them useless as a regression
// gate on a developer machine. The tests below call the scanner's C entry points
// directly with buffers this file controls, so the defect shows up as a wrong value
// rather than as a signal that may or may not arrive.
// ---------------------------------------------------------------------------

mod scanner_abi {
    use std::ffi::{c_char, c_uint, c_void};
    use std::path::{Path, PathBuf};

    /// Mirrors `TREE_SITTER_SERIALIZATION_BUFFER_SIZE` in `tree_sitter/parser.h`.
    pub const SERIALIZATION_BUFFER_SIZE: usize = 1024;

    type CreateFn = unsafe extern "C" fn() -> *mut c_void;
    type DestroyFn = unsafe extern "C" fn(*mut c_void);
    type SerializeFn = unsafe extern "C" fn(*mut c_void, *mut c_char) -> c_uint;
    type DeserializeFn = unsafe extern "C" fn(*mut c_void, *const c_char, c_uint);

    fn library_file_name() -> String {
        if cfg!(target_os = "windows") {
            "tree_sitter_typst.dll".to_string()
        } else if cfg!(target_os = "macos") {
            "libtree_sitter_typst.dylib".to_string()
        } else {
            "libtree_sitter_typst.so".to_string()
        }
    }

    /// Locate the built typst grammar library.
    ///
    /// Honours the same override the registry uses, then falls back to walking the
    /// build directory next to this test binary. ~keep
    fn find_library() -> Option<PathBuf> {
        let file_name = library_file_name();
        if let Ok(dir) = std::env::var("TREE_SITTER_LANGUAGE_PACK_LIBS_DIR") {
            let candidate = Path::new(&dir).join(&file_name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        // ~keep target/<profile>/deps/<test-bin> -> target/<profile>/build/*/out/libs/<lib>
        let exe = std::env::current_exe().ok()?;
        let profile_dir = exe.parent()?.parent()?;
        let entries = std::fs::read_dir(profile_dir.join("build")).ok()?;
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().join("out").join("libs").join(&file_name))
            .find(|candidate| candidate.is_file())
    }

    pub struct Scanner {
        // ~keep Field order is drop order: `payload` frees memory owned by code inside
        // ~keep `_library`, so the library must outlive it.
        payload: *mut c_void,
        destroy: DestroyFn,
        serialize: SerializeFn,
        deserialize: DeserializeFn,
        _library: libloading::Library,
    }

    impl Scanner {
        /// Load the grammar library and create a scanner instance.
        ///
        /// Only the dynamic path exists, and that is deliberate. Under
        /// `TSLP_LINK_MODE=static` the scanner is linked into the test binary as a *local*
        /// symbol -- `nm -gU` on the test executable lists no `external_scanner` symbols at
        /// all -- so there is nothing for the dynamic linker to resolve and a `dlsym`-based
        /// fallback would be dead code. Static builds are handled by the caller instead,
        /// which skips these tests explicitly rather than reporting a false pass. ~keep
        pub fn load() -> Result<Self, String> {
            let path = find_library().ok_or_else(|| format!("no {} next to this test binary", library_file_name()))?;
            // ~keep SAFETY: the path is a grammar library produced by this crate's build script.
            let library = unsafe { libloading::Library::new(&path) }
                .map_err(|e| format!("failed to load {}: {e}", path.display()))?;
            // ~keep SAFETY: every tree-sitter grammar with an external scanner exports these
            // ~keep four symbols with exactly these signatures; a mismatch fails the get below.
            unsafe {
                let create = *library
                    .get::<CreateFn>(b"tree_sitter_typst_external_scanner_create")
                    .map_err(|e| format!("create symbol: {e}"))?;
                let destroy = *library
                    .get::<DestroyFn>(b"tree_sitter_typst_external_scanner_destroy")
                    .map_err(|e| format!("destroy symbol: {e}"))?;
                let serialize = *library
                    .get::<SerializeFn>(b"tree_sitter_typst_external_scanner_serialize")
                    .map_err(|e| format!("serialize symbol: {e}"))?;
                let deserialize = *library
                    .get::<DeserializeFn>(b"tree_sitter_typst_external_scanner_deserialize")
                    .map_err(|e| format!("deserialize symbol: {e}"))?;
                let payload = create();
                if payload.is_null() {
                    return Err("external_scanner_create returned NULL".to_string());
                }
                Ok(Self {
                    payload,
                    destroy,
                    serialize,
                    deserialize,
                    _library: library,
                })
            }
        }

        /// Serialize the current state, returning exactly the bytes the scanner wrote.
        pub fn serialize(&mut self) -> Vec<u8> {
            let mut buffer = vec![0u8; SERIALIZATION_BUFFER_SIZE];
            // ~keep SAFETY: the buffer is TREE_SITTER_SERIALIZATION_BUFFER_SIZE bytes, which is
            // ~keep the contract tree-sitter itself provides to this function.
            let written = unsafe { (self.serialize)(self.payload, buffer.as_mut_ptr().cast::<c_char>()) } as usize;
            assert!(
                written <= SERIALIZATION_BUFFER_SIZE,
                "serialize reported {written} bytes written into a {SERIALIZATION_BUFFER_SIZE}-byte buffer"
            );
            buffer.truncate(written);
            buffer
        }

        /// Hand `bytes` to the scanner as a serialized state of exactly that length.
        pub fn deserialize(&mut self, bytes: &[u8]) {
            // ~keep SAFETY: the pointer is valid for `bytes.len()` bytes, which is precisely the
            // ~keep length passed. A correct scanner reads no further; that is what is under test.
            unsafe {
                (self.deserialize)(
                    self.payload,
                    bytes.as_ptr().cast::<c_char>(),
                    c_uint::try_from(bytes.len()).expect("test buffers are far below u32::MAX"),
                );
            }
        }
    }

    impl Drop for Scanner {
        fn drop(&mut self) {
            // ~keep SAFETY: `payload` came from create() and is destroyed exactly once.
            unsafe { (self.destroy)(self.payload) };
        }
    }

    /// The state a correct scanner holds after `deserialize(.., length = 0)`: an
    /// `indentation` vector holding a single zero, an empty `containers`, and four zero
    /// scalars. Every rejected buffer must land here too. ~keep
    pub fn base_state_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1usize.to_le_bytes()); // ~keep indentation.len == 1
        bytes.extend_from_slice(&0u32.to_le_bytes()); // ~keep indentation[0] == 0
        bytes.extend_from_slice(&0usize.to_le_bytes()); // ~keep containers.len == 0
        bytes.extend_from_slice(&[0, 0, 0, 0]); // ~keep immediate, heading_level, line_start, raw_level
        bytes
    }
}

use scanner_abi::{Scanner, base_state_bytes};

/// A scanner to drive, or `None` when the grammar is linked statically.
///
/// The two failure modes must not be conflated. If typst was never built, these tests are
/// meaningless and must fail loudly -- that is `require_language`. If typst *was* built but
/// its symbols are unreachable, the build is `TSLP_LINK_MODE=static`, where the scanner is a
/// local symbol by construction; the C under test is byte-identical to the dynamic build, so
/// the honest result is a reported skip rather than either a false pass or a false failure.
/// The default link mode is dynamic, so a plain `cargo test` always runs these for real. ~keep
fn scanner_or_skip(test_name: &str) -> Option<Scanner> {
    require_language("typst");
    match Scanner::load() {
        Ok(scanner) => Some(scanner),
        Err(reason) => {
            println!(
                "SKIP {test_name}: typst is built but its external scanner is not dynamically \
                 reachable ({reason}). This is expected under TSLP_LINK_MODE=static; re-run in \
                 the default dynamic mode to exercise the scanner ABI."
            );
            None
        }
    }
}

#[test]
fn empty_state_restores_the_documented_base_state() {
    let Some(mut scanner) = scanner_or_skip("empty_state_restores_the_documented_base_state") else {
        return;
    };
    scanner.deserialize(&[]);

    assert_eq!(
        scanner.serialize(),
        base_state_bytes(),
        "length == 0 must restore the base state; every rejected buffer is asserted against this"
    );
}

#[test]
fn a_full_round_trip_is_a_fixed_point() {
    let Some(mut scanner) = scanner_or_skip("a_full_round_trip_is_a_fixed_point") else {
        return;
    };
    scanner.deserialize(&[]);
    let once = scanner.serialize();

    scanner.deserialize(&once);
    let twice = scanner.serialize();

    // ~keep If deserialize consumed a different number of bytes than serialize wrote, the
    // ~keep two sides disagree on the framing and the second encoding drifts from the first.
    assert_eq!(
        twice, once,
        "serialize/deserialize is not a fixed point, so the framing disagrees"
    );
    assert_eq!(
        once.len(),
        base_state_bytes().len(),
        "base state changed size unexpectedly"
    );
}

/// Encode a scanner state by hand: `indentation`, `containers`, then four scalars.
///
/// Built rather than captured so the tests below can truncate a state whose declared
/// element counts are *not* zero. That distinction is the whole point -- see
/// `every_truncated_prefix_is_rejected_and_falls_back_to_the_base_state`. ~keep
fn encode_state(indentation: &[u32], containers: &[u32], scalars: [u8; 4]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for vector in [indentation, containers] {
        bytes.extend_from_slice(&vector.len().to_le_bytes());
        for value in vector {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
    }
    bytes.extend_from_slice(&scalars);
    bytes
}

/// A state with no zero-valued fields anywhere.
///
/// Every byte that matters is non-zero, so a reader that runs off the end of a
/// truncated copy cannot accidentally reproduce it from zeroed memory. ~keep
fn nontrivial_state() -> Vec<u8> {
    encode_state(&[7, 8, 9], &[4, 5], [1, 2, 1, 3])
}

/// A complete state must round-trip byte-for-byte.
///
/// This is the framing check: if deserialize consumed a different number of bytes than
/// the encoder wrote, the re-encoded state cannot come back identical. ~keep
#[test]
fn a_complete_state_round_trips_byte_for_byte() {
    let state = nontrivial_state();
    let Some(mut scanner) = scanner_or_skip("a_complete_state_round_trips_byte_for_byte") else {
        return;
    };
    scanner.deserialize(&state);

    assert_eq!(
        scanner.serialize(),
        state,
        "a full, valid state did not survive the round trip, so serialize and deserialize \
         disagree about the wire framing"
    );
}

/// Every truncation of a valid state must be rejected, not partially applied.
///
/// This is the test that would have caught the crash on a developer machine. Pre-fix,
/// `vec_u32_deserialize` took no length at all: it read an 8-byte element count and then
/// memcpy'd `count * 4` bytes with no bound against the buffer it was given.
///
/// The state truncated here is deliberately non-trivial. An earlier version of this test
/// truncated the *base* state and passed against the unfixed scanner: the base state is
/// almost entirely zero bytes, so a reader running past the end into zeroed heap memory
/// reconstructed it by coincidence and the assertion never fired. With non-zero element
/// counts the coincidence is impossible -- the count is read from bytes this test
/// controls, so an unbounded reader reports a vector it was never given, on any platform
/// and whatever happens to follow the buffer. ~keep
#[test]
fn every_truncated_prefix_is_rejected_and_falls_back_to_the_base_state() {
    let state = nontrivial_state();
    let expected = base_state_bytes();

    for length in 1..state.len() {
        let Some(mut scanner) = scanner_or_skip("every_truncated_prefix_is_rejected_and_falls_back_to_the_base_state")
        else {
            return;
        };
        scanner.deserialize(&state[..length]);
        let restored = scanner.serialize();

        assert_eq!(
            restored,
            expected,
            "a {length}-byte prefix of a {}-byte state was not rejected: the scanner restored \
             {restored:?} instead of the base state, so it kept reading past the {length} bytes \
             it was given",
            state.len()
        );
    }
}

/// A corrupt element count must never become a memcpy length.
///
/// The count is read straight out of the buffer, so it is only as trustworthy as the
/// buffer. Each case below declares far more elements than the buffer can hold; a
/// scanner that believes the count reads (and, via realloc of a wrapped size, may
/// write) wildly out of bounds. ~keep
#[test]
fn an_element_count_larger_than_the_buffer_is_rejected() {
    let cases: [(&str, u64); 4] = [
        ("u64::MAX", u64::MAX),
        ("wraps when multiplied by 4", u64::MAX / 4 + 1),
        ("larger than the whole serialization buffer", 1 << 20),
        ("one element more than the payload holds", 1),
    ];

    for (label, count) in cases {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&count.to_le_bytes());
        // Deliberately no payload: the buffer ends right after the count it declares.
        let Some(mut scanner) = scanner_or_skip("an_element_count_larger_than_the_buffer_is_rejected") else {
            return;
        };
        scanner.deserialize(&buffer);
        let restored = scanner.serialize();

        assert_eq!(
            restored,
            base_state_bytes(),
            "an element count of {count} ({label}) in an {}-byte buffer was not rejected;              the scanner restored {restored:?}, so it used the count as a memcpy length",
            buffer.len()
        );
    }
}

/// Serialization is all-or-nothing: whatever it emits must deserialize back to itself.
///
/// The previous fix bounded serialize by *truncating* the vectors. Truncation is
/// undetectable to the reader -- a dropped second vector makes the reader take the
/// trailing scalars, and then memory past the buffer, as that vector's length prefix.
/// Returning 0 instead is safe because 0 means "no state", which is a case the reader
/// already handles exactly. ~keep
#[test]
fn serialize_never_emits_a_state_it_cannot_read_back() {
    let Some(mut scanner) = scanner_or_skip("serialize_never_emits_a_state_it_cannot_read_back") else {
        return;
    };
    scanner.deserialize(&[]);

    let emitted = scanner.serialize();
    assert!(
        emitted.len() <= scanner_abi::SERIALIZATION_BUFFER_SIZE,
        "serialize wrote {} bytes into a {}-byte buffer",
        emitted.len(),
        scanner_abi::SERIALIZATION_BUFFER_SIZE
    );

    if !emitted.is_empty() {
        scanner.deserialize(&emitted);
        assert_eq!(
            scanner.serialize(),
            emitted,
            "serialize emitted a state that does not read back as itself, so it is truncated or misframed"
        );
    }
}

/// Nesting deep enough to exercise the serialization size limit must still parse.
///
/// The scanner pushes one `containers` element per open bracket, so nesting is what makes
/// a state outgrow tree-sitter's 1 KiB buffer. That is the path where serialize now returns
/// 0 ("no state") instead of emitting a truncated one, so it is the path most likely to
/// regress parsing. Moderate nesting sits well inside the budget and must parse cleanly;
/// pathological nesting only has to stay memory-safe and terminate. ~keep
#[test]
fn deep_nesting_parses_cleanly_and_never_crashes() {
    require_language("typst");

    let moderate = format!("{}text{}", "[".repeat(50), "]".repeat(50));
    let result = process(&moderate, &ProcessConfig::new("typst"))
        .unwrap_or_else(|error| panic!("typst failed on 50 levels of nesting: {error:?}"));
    assert_eq!(
        result.metrics.error_count, 0,
        "50 levels of nesting is well inside the serialization budget and must parse cleanly"
    );

    // Far past the ~251 elements the 1 KiB buffer can hold, so serialize gives up on the
    // state entirely. The parse must still complete rather than fault. ~keep
    let pathological = format!("{}text{}", "[".repeat(400), "]".repeat(400));
    let result = process(&pathological, &ProcessConfig::new("typst"));
    assert!(
        result.is_ok(),
        "typst must survive 400 levels of nesting even though the scanner state cannot be \
         serialized at that depth: {result:?}"
    );
}
