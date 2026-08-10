```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", { 'language' => 'kotlin' })

```
