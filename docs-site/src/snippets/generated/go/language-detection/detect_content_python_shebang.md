---
id: fixture_go_detect_content_python_shebang
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
	result := tspack.DetectLanguageFromContent(`#!/usr/bin/env python3
pass`)
	fmt.Printf("%+v\n", result)
}
```
