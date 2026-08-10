```go title="Go"
package main

import (
	"encoding/json"
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	var names []string
	if err := json.Unmarshal([]byte(`["zzz_definitely_not_a_real_language_xyz"]`), &names); err != nil {
		panic(fmt.Sprintf("config parse failed: %v", err))
	}
	result, err := tspack.Download(names)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
