---
id: fixture_go_error_handling_unknown_language
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
	"os"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`nonexistent_xyz`),
	}
	_, err := tspack.Process(``, config)
	if err == nil {
		panic("expected call to fail")
	}
	fmt.Fprintf(os.Stderr, "Call failed as expected: %v\n", err)
}
```
