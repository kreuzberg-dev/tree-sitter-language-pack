---
id: fixture_go_detect_ext_gherkin
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
	result := tspack.DetectLanguageFromExtension(`feature`)
	fmt.Printf("%+v\n", result)
}
```
