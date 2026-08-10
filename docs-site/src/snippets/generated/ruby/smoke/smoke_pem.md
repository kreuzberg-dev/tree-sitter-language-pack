```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", { 'language' => 'pem' })

```
