---
id: fixture_go_typescript_function_process_detail
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
		Language: ptr(`typescript`),
	}
	result, err := tspack.Process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n", config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
