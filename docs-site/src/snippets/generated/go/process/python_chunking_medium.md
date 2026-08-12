---
id: fixture_go_python_chunking_medium
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
		Language:     ptr(`python`),
		ChunkMaxSize: 50,
	}
	result, err := tspack.Process(`def first():
    x = 1
    return x

def second():
    y = 2
    return y

def third():
    z = 3
    return z
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
