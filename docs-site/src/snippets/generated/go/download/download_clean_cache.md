```go title="Go"
package main

import (
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	err := tspack.CleanCache()
	if err != nil {
		panic(err)
	}
}
```
