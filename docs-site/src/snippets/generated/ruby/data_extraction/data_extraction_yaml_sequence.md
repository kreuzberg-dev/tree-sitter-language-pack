```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("ports:\n  - 8080\n  - 8081\n", { 'data_extraction' => true, 'language' => 'yaml' })

```
