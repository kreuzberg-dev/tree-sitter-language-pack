```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("function y = hello(x)\ny = x;\nend", { 'language' => 'matlab' })

```
