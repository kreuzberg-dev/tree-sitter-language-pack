```ruby title="Ruby"
require "tree_sitter_language_pack"

TreeSitterLanguagePack.init('{"languages": ["ruby", "python"]}')
TreeSitterLanguagePack.download('{"languages": ["rust", "javascript"]}')

puts TreeSitterLanguagePack.downloaded_languages
```
