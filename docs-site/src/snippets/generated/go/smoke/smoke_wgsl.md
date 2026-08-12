---
id: fixture_go_smoke_wgsl
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
		Language: ptr(`wgsl`),
	}
	result, err := tspack.Process(`@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
