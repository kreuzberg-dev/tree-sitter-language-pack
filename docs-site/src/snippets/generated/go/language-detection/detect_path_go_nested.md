---
id: fixture_go_detect_path_go_nested
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
	result := tspack.DetectLanguageFromPath(`lib/server.go`)
	fmt.Printf("%+v\n", result)
}
```
