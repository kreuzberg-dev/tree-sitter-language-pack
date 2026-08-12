package tspack

import "testing"

// This file exists to make the cgo binding link. `go build ./...` and
// `go test ./...` both archive a library package without ever invoking the
// system linker, so a native library that is missing exported symbols produces
// a green build and fails only when a consumer eventually links a binary. A
// test binary in this package is linked, so every C symbol reachable from the
// calls below must resolve at `go test` time or the build fails outright.
//
// ~keep: the symbols exercised here are deliberate, not arbitrary. They are the
// ones a native library predating the v1.12.0 indents/folds/prefetch additions
// does not export; dropping any of these calls silently restores the blind spot
// this file was added to close.

func TestBindingLinksAgainstNativeLibrary(t *testing.T) {
	t.Parallel()

	if got := LanguageCount(); got == 0 {
		t.Fatalf("LanguageCount() = 0, want a positive count; the native library loaded but reports no languages")
	}
}

func TestPrefetchSymbolIsLinked(t *testing.T) {
	t.Parallel()

	// An empty language list makes this a no-op at runtime; the value is that
	// ts_pack_prefetch must exist for this test binary to link at all.
	//
	// ~keep: the slice must be non-nil. Prefetch marshals its argument to JSON,
	// and a nil slice becomes `null`, which the FFI rejects with "invalid type:
	// null, expected a sequence" — an argument-encoding bug, not a link failure.
	if err := Prefetch([]string{}); err != nil {
		t.Fatalf("Prefetch([]string{}) = %v, want nil", err)
	}
}

func TestIndentsAndFoldsQuerySymbolsAreLinked(t *testing.T) {
	t.Parallel()

	for _, language := range []string{"python", "rust", "go"} {
		if !HasLanguage(language) {
			continue
		}
		// A language may legitimately ship no indents/folds queries, so a nil
		// result is acceptable; an unlinkable symbol is not, and is caught by
		// the linker before this test ever runs.
		_ = GetIndentsQuery(language)
		_ = GetFoldsQuery(language)
		return
	}

	t.Fatal("none of python/rust/go are available; cannot exercise indents/folds query symbols")
}
