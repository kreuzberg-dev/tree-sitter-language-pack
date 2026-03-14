package tspack

import (
	"testing"
)

func TestNewRegistry(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	if r.ptr == nil {
		t.Fatal("NewRegistry() returned nil ptr")
	}
}

func TestLanguageCount(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	count := r.LanguageCount()
	if count <= 0 {
		t.Fatalf("LanguageCount() = %d, want > 0", count)
	}
	t.Logf("LanguageCount() = %d", count)
}

func TestAvailableLanguages(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	languages := r.AvailableLanguages()
	if len(languages) == 0 {
		t.Fatal("AvailableLanguages() returned empty slice")
	}

	count := r.LanguageCount()
	if len(languages) != count {
		t.Errorf("AvailableLanguages() returned %d items, LanguageCount() = %d", len(languages), count)
	}

	// Verify names are non-empty strings.
	for i, name := range languages {
		if name == "" {
			t.Errorf("AvailableLanguages()[%d] is empty", i)
		}
	}

	t.Logf("First 5 languages: %v", languages[:min(5, len(languages))])
}

func TestHasLanguage(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	// Pick the first available language to test with.
	languages := r.AvailableLanguages()
	if len(languages) == 0 {
		t.Fatal("no languages available")
	}

	name := languages[0]
	if !r.HasLanguage(name) {
		t.Errorf("HasLanguage(%q) = false, want true", name)
	}

	if r.HasLanguage("this-language-does-not-exist-at-all") {
		t.Error("HasLanguage(nonexistent) = true, want false")
	}
}

func TestGetLanguage(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	// Pick the first available language to test with.
	languages := r.AvailableLanguages()
	if len(languages) == 0 {
		t.Fatal("no languages available")
	}

	name := languages[0]
	ptr, err := r.GetLanguage(name)
	if err != nil {
		t.Fatalf("GetLanguage(%q) error: %v", name, err)
	}
	if ptr == nil {
		t.Fatalf("GetLanguage(%q) returned nil pointer", name)
	}

	t.Logf("GetLanguage(%q) = %v", name, ptr)
}

func TestGetLanguageNotFound(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	ptr, err := r.GetLanguage("this-language-does-not-exist-at-all")
	if err == nil {
		t.Fatal("GetLanguage(nonexistent) expected error, got nil")
	}
	if ptr != nil {
		t.Fatal("GetLanguage(nonexistent) expected nil pointer, got non-nil")
	}

	t.Logf("GetLanguage(nonexistent) error: %v", err)
}

func TestClosedRegistryReturnsErrors(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	r.Close()

	// LanguageCount should return 0 on a closed registry.
	if count := r.LanguageCount(); count != 0 {
		t.Errorf("LanguageCount() on closed registry = %d, want 0", count)
	}

	// HasLanguage should return false on a closed registry.
	if r.HasLanguage("python") {
		t.Error("HasLanguage() on closed registry = true, want false")
	}

	// GetLanguage should return an error on a closed registry.
	ptr, err := r.GetLanguage("python")
	if err == nil {
		t.Error("GetLanguage() on closed registry expected error, got nil")
	}
	if ptr != nil {
		t.Error("GetLanguage() on closed registry expected nil pointer")
	}

	// LanguageNameAt should return an error on a closed registry.
	name, err := r.LanguageNameAt(0)
	if err == nil {
		t.Error("LanguageNameAt() on closed registry expected error, got nil")
	}
	if name != "" {
		t.Errorf("LanguageNameAt() on closed registry = %q, want empty", name)
	}

	// AvailableLanguages should return nil on a closed registry.
	if langs := r.AvailableLanguages(); langs != nil {
		t.Errorf("AvailableLanguages() on closed registry = %v, want nil", langs)
	}
}

func TestDoubleCloseIsSafe(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}

	// Calling Close multiple times must not panic or double-free.
	r.Close()
	r.Close()
}

func TestLanguageNameAtOutOfBounds(t *testing.T) {
	r, err := NewRegistry()
	if err != nil {
		t.Fatalf("NewRegistry() error: %v", err)
	}
	defer r.Close()

	count := r.LanguageCount()

	// Exactly at count (one past the end).
	_, err = r.LanguageNameAt(count)
	if err == nil {
		t.Error("LanguageNameAt(count) expected error, got nil")
	}

	// Well beyond bounds.
	_, err = r.LanguageNameAt(count + 1000)
	if err == nil {
		t.Error("LanguageNameAt(count+1000) expected error, got nil")
	}

	// Negative index (will be a very large uintptr_t).
	_, err = r.LanguageNameAt(-1)
	if err == nil {
		t.Error("LanguageNameAt(-1) expected error, got nil")
	}
}
