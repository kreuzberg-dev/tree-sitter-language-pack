```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("config FOO\n\tbool \"Enable foo\"", { 'language' => 'kconfig' })

```
