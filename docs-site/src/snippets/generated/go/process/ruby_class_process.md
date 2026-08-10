```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`ruby`),
	}
	result, err := tspack.Process(`require 'json'

class Greeter
  def greet(name)
    "Hello #{name}"
  end
end
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
