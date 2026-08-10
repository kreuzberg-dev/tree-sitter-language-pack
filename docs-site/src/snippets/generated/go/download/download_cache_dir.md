```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	result, err := tspack.CacheDir()
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
