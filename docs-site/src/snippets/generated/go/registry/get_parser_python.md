---
id: fixture_go_get_parser_python
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
	parser, err := tspack.GetParser(`python`)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", parser)
}
```
