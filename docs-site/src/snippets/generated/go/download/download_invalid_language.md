---
id: fixture_go_download_invalid_language
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
	"os"
)

func main() {
	var names []string
	if err := json.Unmarshal([]byte(`["zzz_definitely_not_a_real_language_xyz"]`), &names); err != nil {
		panic(fmt.Sprintf("config parse failed: %v", err))
	}
	_, err := tspack.Download(names)
	if err == nil {
		panic("expected call to fail")
	}
	fmt.Fprintf(os.Stderr, "Call failed as expected: %v\n", err)
}
```
