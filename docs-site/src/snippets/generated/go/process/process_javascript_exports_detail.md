```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`javascript`),
	}
	result, err := tspack.Process("export function greet(name) {\n  return `Hello ${name}`;\n}\n\nexport const VERSION = '1.0';\n", config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
