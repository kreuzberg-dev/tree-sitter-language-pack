---
id: fixture_go_download_init_default
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
	config := tspack.PackConfig{}
	err := tspack.Init(config)
	if err != nil {
		panic(err)
	}
}
```
