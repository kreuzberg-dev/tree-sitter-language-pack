```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	language, err := tspack.GetLanguage(`shell`)
	if err != nil {
		panic(err)
	}
	fmt.Println(language)
}
```
