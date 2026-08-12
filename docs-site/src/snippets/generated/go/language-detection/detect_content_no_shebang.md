---
id: fixture_go_detect_content_no_shebang
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
	result := tspack.DetectLanguageFromContent(`no shebang here`)
	fmt.Printf("%+v\n", result)
}
```
