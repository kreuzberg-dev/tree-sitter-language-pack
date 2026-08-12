---
id: fixture_go_data_extraction_dtd_element_decl
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
		Language:       ptr(`dtd`),
		DataExtraction: true,
	}
	result, err := tspack.Process(`<!ELEMENT server (host, port)>
<!ELEMENT host (#PCDATA)>
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
