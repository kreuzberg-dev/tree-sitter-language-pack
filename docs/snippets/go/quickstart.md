```go title="Go"
package main

import (
    "fmt"
    "log"
    tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
    registry, _ := tslp.NewRegistry()
    defer registry.Close()

    tree, _ := registry.ParseString("go", "package main\nfunc hello() {}")
    defer tree.Close()

    rootType, err := tree.RootNodeType()
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println("Root:", rootType)
}
```
