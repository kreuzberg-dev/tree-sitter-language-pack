```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("Root = Item*\nItem = 'token'", { 'language' => 'ungrammar' })

```
