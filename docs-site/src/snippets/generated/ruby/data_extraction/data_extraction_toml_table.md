```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("[server]\nhost = \"localhost\"\nport = 8080\n", { 'data_extraction' => true, 'language' => 'toml' })

```
