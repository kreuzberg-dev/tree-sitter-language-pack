```go title="Go"
package main

import (
	"encoding/json"
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	var languages []string
	if err := json.Unmarshal([]byte(`[]`), &languages); err != nil {
		panic(fmt.Sprintf("config parse failed: %v", err))
	}
	err := tspack.Prefetch(languages)
	if err != nil {
		panic(err)
	}
}
```
