package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func TestMain(m *testing.M) {
	// Run tests — static/dev builds have languages compiled in,
	// no Init/download needed.
	os.Exit(m.Run())
}

type BasicFixture struct {
	Name             string   `json:"name"`
	Test             string   `json:"test"`
	Language         string   `json:"language,omitempty"`
	Expected         *bool    `json:"expected,omitempty"`
	ExpectedMin      *int     `json:"expected_min,omitempty"`
	ExpectedContains []string `json:"expected_contains,omitempty"`
}

func loadBasicFixtures(t *testing.T) []BasicFixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "fixtures", "basic.json"))
	if err != nil {
		t.Fatalf("Failed to read basic.json: %v", err)
	}
	var fixtures []BasicFixture
	if err := json.Unmarshal(data, &fixtures); err != nil {
		t.Fatalf("Failed to parse basic.json: %v", err)
	}
	return fixtures
}

func TestBasicFixtures(t *testing.T) {
	registry, err := tslp.NewRegistry()
	if err != nil {
		t.Fatalf("Failed to create registry: %v", err)
	}
	defer registry.Close()

	fixtures := loadBasicFixtures(t)
	for _, fixture := range fixtures {
		t.Run(fixture.Name, func(t *testing.T) {
			switch fixture.Test {
			case "language_count":
				count := registry.LanguageCount()
				if count < *fixture.ExpectedMin {
					t.Errorf("language_count %d < expected min %d", count, *fixture.ExpectedMin)
				}
			case "has_language":
				result := registry.HasLanguage(fixture.Language)
				if result != *fixture.Expected {
					t.Errorf("has_language(%q) = %v, expected %v", fixture.Language, result, *fixture.Expected)
				}
			case "available_languages":
				langs := registry.AvailableLanguages()
				langSet := make(map[string]bool)
				for _, l := range langs {
					langSet[l] = true
				}
				for _, expected := range fixture.ExpectedContains {
					if !langSet[expected] {
						t.Errorf("available_languages missing %q", expected)
					}
				}
			default:
				t.Fatalf("Unknown test type: %s", fixture.Test)
			}
		})
	}
}

func TestParseValidation(t *testing.T) {
	registry, err := tslp.NewRegistry()
	if err != nil {
		t.Fatalf("Failed to create registry: %v", err)
	}
	defer registry.Close()

	t.Run("parses_python_code", func(t *testing.T) {
		tree, err := registry.ParseString("python", "def hello(): pass\n")
		if err != nil {
			t.Fatalf("ParseString failed: %v", err)
		}
		defer tree.Close()

		nodeType, err := tree.RootNodeType()
		if err != nil {
			t.Fatalf("RootNodeType failed: %v", err)
		}
		if nodeType != "module" {
			t.Errorf("root node type = %q, expected %q", nodeType, "module")
		}
		childCount, err := tree.RootChildCount()
		if err != nil {
			t.Fatalf("RootChildCount failed: %v", err)
		}
		if childCount < 1 {
			t.Error("root child count < 1")
		}
		hasErrors, err := tree.HasErrorNodes()
		if err != nil {
			t.Fatalf("HasErrorNodes failed: %v", err)
		}
		if hasErrors {
			t.Error("tree has error nodes")
		}
	})

	t.Run("errors_on_invalid_language", func(t *testing.T) {
		_, err := registry.ParseString("nonexistent_xyz_123", "code")
		if err == nil {
			t.Error("expected error for invalid language, got nil")
		}
	})
}

func TestDownloadAPI(t *testing.T) {
	t.Run("downloaded_languages_returns_list", func(t *testing.T) {
		langs, err := tslp.DownloadedLanguages()
		if err != nil {
			t.Fatalf("DownloadedLanguages failed: %v", err)
		}
		if len(langs) == 0 {
			t.Error("downloaded_languages returned empty list")
		}
	})

	t.Run("cache_dir_returns_string", func(t *testing.T) {
		cacheDir, err := tslp.CacheDir()
		if err != nil {
			t.Fatalf("CacheDir failed: %v", err)
		}
		if cacheDir == "" {
			t.Error("cache_dir returned empty string")
		}
	})

	t.Run("manifest_languages_returns_170_plus", func(t *testing.T) {
		langs, err := tslp.ManifestLanguages()
		if err != nil {
			t.Fatalf("ManifestLanguages failed: %v", err)
		}
		if len(langs) < 170 {
			t.Errorf("manifest_languages returned %d items, expected >= 170", len(langs))
		}
	})
}

func TestErrorHandling(t *testing.T) {
	registry, err := tslp.NewRegistry()
	if err != nil {
		t.Fatalf("Failed to create registry: %v", err)
	}
	defer registry.Close()

	t.Run("process_with_invalid_language_errors", func(t *testing.T) {
		config := tslp.ProcessConfig{Language: "nonexistent_xyz"}
		_, err := registry.Process("code", config)
		if err == nil {
			t.Error("expected error for invalid language in process, got nil")
		}
	})

	t.Run("has_language_nonexistent_returns_false", func(t *testing.T) {
		result := registry.HasLanguage("nonexistent_lang_xyz_123")
		if result {
			t.Error("has_language for nonexistent language should return false")
		}
	})
}
