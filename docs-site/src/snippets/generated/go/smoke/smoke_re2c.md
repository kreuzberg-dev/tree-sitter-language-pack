```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`re2c`),
	}
	result, err := tspack.Process(`/*!re2c
  [a-z]+ { return; }
*/`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
