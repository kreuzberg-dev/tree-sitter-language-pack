```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	parser, err := tspack.GetParser(`python`)
	if err != nil {
		panic(err)
	}
	fmt.Println(parser)
}
```
