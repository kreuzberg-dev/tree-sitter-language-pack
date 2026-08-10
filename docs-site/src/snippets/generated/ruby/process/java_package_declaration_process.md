```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", { 'language' => 'java' })

```
