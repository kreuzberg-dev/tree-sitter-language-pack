```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("%token EOF\n%%\n", { 'language' => 'menhir' })

```
