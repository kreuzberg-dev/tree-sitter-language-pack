```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("export function greet(name) {\n  return `Hello ${name}`;\n}\n\nexport const VERSION = '1.0';\n", { 'language' => 'javascript' })

```
