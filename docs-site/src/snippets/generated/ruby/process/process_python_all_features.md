```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("import os\nfrom pathlib import Path\n\n\# Configuration\nMY_CONST = 42\n\ndef process_file(path):\n    \"\"\"Process a file and return contents.\"\"\"\n    with open(path) as f:\n        return f.read()\n\nclass FileProcessor:\n    def __init__(self, base_dir):\n        self.base_dir = base_dir\n", { 'comments' => true, 'docstrings' => true, 'imports' => true, 'language' => 'python', 'structure' => true, 'symbols' => true })

```
