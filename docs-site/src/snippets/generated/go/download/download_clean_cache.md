---
id: fixture_go_download_clean_cache
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	err := tspack.CleanCache()
	if err != nil {
		panic(err)
	}
}
```
