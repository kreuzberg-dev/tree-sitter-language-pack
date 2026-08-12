---
id: fixture_go_smoke_lean
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
		Language: ptr(`lean`),
	}
	result, err := tspack.Process(`def main : IO Unit := pure ()`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
