```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`sflog`),
	}
	result, err := tspack.Process(`37.0 APEX_CODE,DEBUG
16:06:58.18 (1)|EXECUTION_STARTED
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
