```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n", { 'language' => 'typescript' })

```
