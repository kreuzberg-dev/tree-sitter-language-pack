```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("\# module docstring\nimport os\n\ndef hello():\n    \# greeting\n    print('hello')\n\ndef world():\n    print('world')\n", { 'language' => 'python' })

```
