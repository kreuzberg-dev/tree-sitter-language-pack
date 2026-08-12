---
id: fixture_go_download_manifest_languages
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
	result, err := tspack.ManifestLanguages()
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
