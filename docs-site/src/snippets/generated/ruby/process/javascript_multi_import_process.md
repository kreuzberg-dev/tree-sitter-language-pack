```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("import fs from 'fs';\nimport path from 'path';\n\nfunction process(input) {\n    return input.trim();\n}\n", { 'language' => 'javascript' })

```
