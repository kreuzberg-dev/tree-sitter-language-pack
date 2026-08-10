```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	result := tspack.DetectLanguageFromContent(`#!/usr/bin/env python3
pass`)
	fmt.Println(result)
}
```
