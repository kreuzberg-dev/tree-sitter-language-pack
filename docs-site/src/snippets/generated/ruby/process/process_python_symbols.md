```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", { 'language' => 'python', 'symbols' => true })

```
