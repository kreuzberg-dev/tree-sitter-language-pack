```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("[database]\nhost=localhost\nport=5432\n", { 'data_extraction' => true, 'language' => 'ini' })

```
