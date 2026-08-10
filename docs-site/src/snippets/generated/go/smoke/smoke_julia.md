```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`julia`),
	}
	result, err := tspack.Process(`function main() end`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
