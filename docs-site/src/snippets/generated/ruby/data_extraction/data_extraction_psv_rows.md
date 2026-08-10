```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("a|b|c\n1|2|3\n", { 'data_extraction' => true, 'language' => 'psv' })

```
