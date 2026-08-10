```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("def broken(\n    return\nclass", { 'diagnostics' => true, 'language' => 'python' })

```
