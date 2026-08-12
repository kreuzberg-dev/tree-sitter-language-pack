---
id: fixture_go_process_rust_structure_name
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
		Language: ptr(`rust`),
	}
	result, err := tspack.Process(`pub struct MyConfig {
    pub name: String,
    pub value: i32,
}

impl MyConfig {
    pub fn new() -> Self {
        Self { name: String::new(), value: 0 }
    }
}
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", result)
}
```
