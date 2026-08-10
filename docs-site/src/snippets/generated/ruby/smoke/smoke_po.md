```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("msgid \"hello\"\nmsgstr \"world\"", { 'language' => 'po' })

```
