```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('SECTIONS { .text : { *(.text) } }', { 'language' => 'linkerscript' })

```
