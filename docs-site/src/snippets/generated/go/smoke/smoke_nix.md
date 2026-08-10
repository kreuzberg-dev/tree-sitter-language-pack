```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`nix`),
	}
	result, err := tspack.Process(`{ pkgs ? import <nixpkgs> {} }: pkgs.hello`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
