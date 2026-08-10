```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", { 'chunk_max_size' => 30, 'language' => 'rust' })

```
