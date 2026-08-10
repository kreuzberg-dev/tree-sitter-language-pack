```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language:       ptr(`hcl`),
		DataExtraction: true,
	}
	result, err := tspack.Process(`region = "us-east-1"
count  = 3
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
