```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`llvm`),
	}
	result, err := tspack.Process(`define i32 @main() { ret i32 0 }`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
