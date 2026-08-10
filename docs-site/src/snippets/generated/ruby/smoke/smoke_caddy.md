```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process(":8080 {\n\trespond \"Hello\"\n}", { 'language' => 'caddy' })

```
