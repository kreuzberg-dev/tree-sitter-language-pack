```go title="Go"
package main

import (
    "fmt"
    tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
    tslp.Init(`{"languages": ["go", "python"]}`)
    tslp.Download([]string{"rust", "javascript"})
    languages, _ := tslp.DownloadedLanguages()
    fmt.Println(languages)
}
```
