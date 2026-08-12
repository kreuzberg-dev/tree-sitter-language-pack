---
id: fixture_go_process_python_all_features
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language:   ptr(`python`),
		Structure:  true,
		Imports:    true,
		Comments:   true,
		Docstrings: true,
		Symbols:    true,
	}
	result, err := tspack.Process(`import os
from pathlib import Path

# Configuration
MY_CONST = 42

def process_file(path):
    """Process a file and return contents."""
    with open(path) as f:
        return f.read()

class FileProcessor:
    def __init__(self, base_dir):
        self.base_dir = base_dir
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
