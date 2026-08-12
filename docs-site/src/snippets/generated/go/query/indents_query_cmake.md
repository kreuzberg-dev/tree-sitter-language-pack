---
id: fixture_go_indents_query_cmake
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
	result := tspack.GetIndentsQuery(`cmake`)
	fmt.Printf("%+v\n", result)
}
```
