```go title="Go"
package main

import (
	"fmt"
	tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func ptr[T any](value T) *T { return &value }
func main() {
	config := tspack.ProcessConfig{
		Language: ptr(`gherkin`),
	}
	result, err := tspack.Process(`Feature: Calculator
  Scenario: Add numbers
    Given I have entered 1
    When I add 2
    Then the result should be 3
`, config)
	if err != nil {
		panic(err)
	}
	fmt.Println(result)
}
```
