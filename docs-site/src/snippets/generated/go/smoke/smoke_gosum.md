```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`gosum`),
	}
	result, err := tspack.Process(`example.com/pkg v1.0.0 h1:abc=`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
