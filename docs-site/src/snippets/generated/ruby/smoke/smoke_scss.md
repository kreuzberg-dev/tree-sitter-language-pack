```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("$color: red;\nbody { color: $color; }", { 'language' => 'scss' })

```
