```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("/*!re2c\n  [a-z]+ { return; }\n*/", { 'language' => 're2c' })

```
