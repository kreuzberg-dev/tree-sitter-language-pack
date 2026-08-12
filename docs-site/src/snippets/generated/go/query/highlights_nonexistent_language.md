---
id: fixture_go_highlights_nonexistent_language
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
	result := tspack.GetHighlightsQuery(`zzz_nonexistent_lang`)
	fmt.Printf("%+v\n", result)
}
```
