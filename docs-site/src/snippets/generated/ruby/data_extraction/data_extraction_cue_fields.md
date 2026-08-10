```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("host: \"localhost\"\nport: 8080\n", { 'data_extraction' => true, 'language' => 'cue' })

```
