```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('SELECT ?s WHERE { ?s ?p ?o }', { 'language' => 'sparql' })

```
