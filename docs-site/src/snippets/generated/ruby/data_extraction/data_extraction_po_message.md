```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", { 'data_extraction' => true, 'language' => 'po' })

```
