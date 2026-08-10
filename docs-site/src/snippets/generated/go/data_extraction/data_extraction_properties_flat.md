```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language:       ptr(`properties`),
		DataExtraction: true,
	}
	result, err := tspack.Process(`host=localhost
port=8080
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
