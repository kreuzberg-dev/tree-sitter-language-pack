```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", { 'language' => 'javascript' })

```
