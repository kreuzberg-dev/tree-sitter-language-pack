```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n", { 'language' => 'python' })

```
