---
id: fixture_go_detect_path_dotfile
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
	result := tspack.DetectLanguageFromPath(`.gitignore`)
	fmt.Printf("%+v\n", result)
}
```
