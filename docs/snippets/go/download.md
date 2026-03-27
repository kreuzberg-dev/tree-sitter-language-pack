```go title="Go"
package main

import (
    "fmt"
    tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go/v1"
)

func main() {
    tslp.Init(`{"languages": ["go", "python"]}`)
    tslp.Download([]string{"rust", "javascript"})
    languages, _ := tslp.DownloadedLanguages()
    fmt.Println(languages)
}
```
