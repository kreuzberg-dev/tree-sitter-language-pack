---
id: fixture_go_registry_has_language_true
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
	result := tspack.HasLanguage(`python`)
	fmt.Printf("%+v\n", result)
}
```
