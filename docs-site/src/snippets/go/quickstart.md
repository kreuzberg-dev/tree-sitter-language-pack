```go title="Go"
package main

import (
	"fmt"
	"log"

	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	parser, err := tspack.GetParser("go")
	if err != nil {
		log.Fatal(err)
	}
	defer parser.Free()

	tree := parser.Parse("package main\nfunc hello() {}")
	defer tree.Free()

	root := tree.RootNode()
	defer root.Free()

	fmt.Println("Root:", root.Kind())
}
```
