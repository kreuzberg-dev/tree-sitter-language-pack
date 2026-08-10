```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("server.host=localhost\nserver.port=8080\n", { 'data_extraction' => true, 'language' => 'properties' })

```
