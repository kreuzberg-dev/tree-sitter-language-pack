```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`postscript`),
	}
	result, err := tspack.Process(`/hello { (Hello) show } def`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
