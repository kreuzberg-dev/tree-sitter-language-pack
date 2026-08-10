```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('function greet(name: string): string { return `hi ${name}`; }', { 'language' => 'typescript' })

```
