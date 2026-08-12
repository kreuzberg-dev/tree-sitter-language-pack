---
id: fixture_go_data_extraction_hcl_block
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
		Language:       ptr(`hcl`),
		DataExtraction: true,
	}
	result, err := tspack.Process(`resource "aws_instance" "web" {
  ami = "ami-123"
  instance_type = "t2.micro"
}
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
