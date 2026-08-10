```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("open(\"/x\", O_RDONLY) = 3\n", { 'language' => 'strace' })

```
