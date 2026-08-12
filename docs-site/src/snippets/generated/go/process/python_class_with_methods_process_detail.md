---
id: fixture_go_python_class_with_methods_process_detail
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
		Language: ptr(`python`),
	}
	result, err := tspack.Process(`class Calculator:
    def add(self, a, b):
        return a + b

    def subtract(self, a, b):
        return a - b
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
