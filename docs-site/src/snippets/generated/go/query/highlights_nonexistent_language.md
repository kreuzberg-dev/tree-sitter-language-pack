```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	result := tspack.GetHighlightsQuery(`zzz_nonexistent_lang`)
	fmt.Println(result)
}
```
