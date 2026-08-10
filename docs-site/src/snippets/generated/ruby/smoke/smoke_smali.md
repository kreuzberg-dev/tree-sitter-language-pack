```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process(".class public LMain;\n.super Ljava/lang/Object;", { 'language' => 'smali' })

```
