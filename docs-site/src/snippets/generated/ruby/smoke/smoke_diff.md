```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", { 'language' => 'diff' })

```
