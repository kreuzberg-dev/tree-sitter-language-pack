```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("MATCH (n) RETURN n\n", { 'language' => 'cypher' })

```
