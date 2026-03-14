package main

import (
	"fmt"

	tspack "github.com/kreuzberg-dev/tree-sitter-language-pack/go"
)

func main() {
	reg, err := tspack.NewRegistry()
	if err != nil {
		panic(err)
	}
	defer reg.Close()

	count := reg.LanguageCount()
	fmt.Printf("Available languages: %d\n", count)

	if count == 0 {
		panic("no languages available")
	}
	if !reg.HasLanguage("go") {
		panic("go not found")
	}

	fmt.Println("Go smoke test passed")
}
