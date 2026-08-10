```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('void main() { gl_Position = vec4(0.0); }', { 'language' => 'glsl' })

```
