```go title="Go"
package main

import (
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.PackConfig{
		CacheDir: ptr(`/tmp/tslp_test_cache`),
	}
	err := tspack.Configure(config)
	if err != nil {
		panic(err)
	}
}
```
