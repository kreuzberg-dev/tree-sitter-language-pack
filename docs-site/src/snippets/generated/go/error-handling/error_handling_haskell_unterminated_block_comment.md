---
id: fixture_go_error_handling_haskell_unterminated_block_comment
language: go
target: go
level: typecheck
requires: []
side_effect: safe
---

```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`haskell`),
	}
	result, err := tspack.Process(`{-aaaaaaaaaaaaaa aaaa}
    {-aaa (aaaaaaaaaa [aaaaaaaaaaaaa aaa`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
