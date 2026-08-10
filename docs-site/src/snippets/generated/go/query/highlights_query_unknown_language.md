```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	result := tspack.GetHighlightsQuery(`nonexistent_language_xyz`)
	fmt.Println(result)
}
```
