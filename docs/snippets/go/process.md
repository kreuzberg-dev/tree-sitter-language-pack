```go title="Go"
package main

import (
    "fmt"
    tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
    registry, _ := tslp.NewRegistry()
    defer registry.Close()

    config := tslp.ProcessConfig{
        Language:  "go",
        Structure: true,
        Imports:   true,
    }
    result, _ := registry.Process(
        "package main\nimport \"fmt\"\nfunc hello() { fmt.Println(\"hi\") }",
        config,
    )
    fmt.Println("Language:", result.Language)
    fmt.Println("Structure:", result.Structure)
    fmt.Println("Metrics:", result.Metrics)
}
```
