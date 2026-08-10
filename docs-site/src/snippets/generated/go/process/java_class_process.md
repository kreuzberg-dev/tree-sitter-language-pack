```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`java`),
	}
	result, err := tspack.Process(`import java.util.List;

public class Greeter {
    public String greet(String name) {
        return "Hello " + name;
    }
}
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
