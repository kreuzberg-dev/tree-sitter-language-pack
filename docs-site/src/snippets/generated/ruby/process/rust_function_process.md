```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", { 'language' => 'rust' })

```
