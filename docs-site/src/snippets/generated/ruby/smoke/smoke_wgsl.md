```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }', { 'language' => 'wgsl' })

```
