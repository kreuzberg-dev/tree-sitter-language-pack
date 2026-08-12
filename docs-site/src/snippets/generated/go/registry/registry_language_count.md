---
id: fixture_go_registry_language_count
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
	result := tspack.LanguageCount()
	fmt.Printf("%+v\n", result)
}
```
