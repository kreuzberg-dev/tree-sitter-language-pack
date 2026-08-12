---
id: fixture_go_python_chunking_process_detail
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
		ChunkMaxSize: 30,
	}
	result, err := tspack.Process(`def alpha():
    pass

def beta():
    pass

def gamma():
    pass

def delta():
    pass
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
