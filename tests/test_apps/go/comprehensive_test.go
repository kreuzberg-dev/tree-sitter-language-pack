package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go/v1"
)

type ProcessFixture struct {
	Name     string                 `json:"name"`
	Test     string                 `json:"test"`
	Source   string                 `json:"source"`
	Config   map[string]interface{} `json:"config"`
	Expected map[string]interface{} `json:"expected"`
}

func loadProcessFixtures(t *testing.T, filename string) []ProcessFixture {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "fixtures", filename))
	if err != nil {
		t.Fatalf("Failed to read %s: %v", filename, err)
	}
	var fixtures []ProcessFixture
	if err := json.Unmarshal(data, &fixtures); err != nil {
		t.Fatalf("Failed to parse %s: %v", filename, err)
	}
	return fixtures
}

func TestProcessFixtures(t *testing.T) {
	registry, err := tslp.NewRegistry()
	if err != nil {
		t.Fatalf("Failed to create registry: %v", err)
	}
	defer registry.Free()

	fixtures := loadProcessFixtures(t, "process.json")
	for _, fixture := range fixtures {
		t.Run(fixture.Name, func(t *testing.T) {
			configJSON, _ := json.Marshal(fixture.Config)
			result, err := registry.Process(fixture.Source, string(configJSON))
			if err != nil {
				t.Fatalf("process() failed: %v", err)
			}

			var resultMap map[string]interface{}
			if err := json.Unmarshal([]byte(result), &resultMap); err != nil {
				t.Fatalf("Failed to parse result: %v", err)
			}

			if lang, ok := fixture.Expected["language"]; ok {
				if resultMap["language"] != lang {
					t.Errorf("language = %v, expected %v", resultMap["language"], lang)
				}
			}
			if minVal, ok := fixture.Expected["structure_min"]; ok {
				structures := resultMap["structure"].([]interface{})
				min := int(minVal.(float64))
				if len(structures) < min {
					t.Errorf("structure count %d < min %d", len(structures), min)
				}
			}
			if minVal, ok := fixture.Expected["imports_min"]; ok {
				imports := resultMap["imports"].([]interface{})
				min := int(minVal.(float64))
				if len(imports) < min {
					t.Errorf("imports count %d < min %d", len(imports), min)
				}
			}
		})
	}
}

func TestChunkingFixtures(t *testing.T) {
	registry, err := tslp.NewRegistry()
	if err != nil {
		t.Fatalf("Failed to create registry: %v", err)
	}
	defer registry.Free()

	fixtures := loadProcessFixtures(t, "chunking.json")
	for _, fixture := range fixtures {
		t.Run(fixture.Name, func(t *testing.T) {
			configJSON, _ := json.Marshal(fixture.Config)
			result, err := registry.Process(fixture.Source, string(configJSON))
			if err != nil {
				t.Fatalf("process() failed: %v", err)
			}

			var resultMap map[string]interface{}
			if err := json.Unmarshal([]byte(result), &resultMap); err != nil {
				t.Fatalf("Failed to parse result: %v", err)
			}

			if minVal, ok := fixture.Expected["chunks_min"]; ok {
				chunks := resultMap["chunks"].([]interface{})
				min := int(minVal.(float64))
				if len(chunks) < min {
					t.Errorf("chunks count %d < min %d", len(chunks), min)
				}
			}
		})
	}
}

func TestSetup(t *testing.T) {
	t.Run("init_with_multiple_languages", func(t *testing.T) {
		configJSON := `{"languages":["python","javascript","rust","go","ruby","java","c","cpp"]}`
		err := tslp.Init(configJSON)
		if err != nil {
			t.Fatalf("Init with multiple languages failed: %v", err)
		}
	})
}
