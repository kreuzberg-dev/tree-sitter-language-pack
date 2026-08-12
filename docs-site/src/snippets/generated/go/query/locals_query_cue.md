---
id: fixture_go_locals_query_cue
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

func main() {
	result := tspack.GetLocalsQuery(`cue`)
	fmt.Printf("%+v\n", result)
}
```
