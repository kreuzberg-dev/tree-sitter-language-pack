```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", { 'language' => 'latex' })

```
