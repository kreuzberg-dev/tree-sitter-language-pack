```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", { 'language' => 'python' })

```
