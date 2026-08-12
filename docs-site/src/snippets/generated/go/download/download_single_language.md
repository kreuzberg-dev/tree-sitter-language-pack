---
id: fixture_go_download_single_language
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	"encoding/json"
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	var names []string
	if err := json.Unmarshal([]byte(`["python"]`), &names); err != nil {
		panic(fmt.Sprintf("config parse failed: %v", err))
	}
	result, err := tspack.Download(names)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
