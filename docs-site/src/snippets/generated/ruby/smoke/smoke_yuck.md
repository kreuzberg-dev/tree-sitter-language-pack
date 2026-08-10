```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('(defwidget main [] (label :text "hi"))', { 'language' => 'yuck' })

```
