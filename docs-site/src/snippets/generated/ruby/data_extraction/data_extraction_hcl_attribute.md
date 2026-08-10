```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("region = \"us-east-1\"\ncount  = 3\n", { 'data_extraction' => true, 'language' => 'hcl' })

```
