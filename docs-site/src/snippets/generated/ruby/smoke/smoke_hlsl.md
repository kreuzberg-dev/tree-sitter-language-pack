```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('float4 main() : SV_Target { return 0; }', { 'language' => 'hlsl' })

```
