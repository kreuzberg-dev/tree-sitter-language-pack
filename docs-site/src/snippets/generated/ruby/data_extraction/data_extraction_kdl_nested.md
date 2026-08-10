```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("server {\n  host \"localhost\"\n  port 8080\n}\n", { 'data_extraction' => true, 'language' => 'kdl' })

```
