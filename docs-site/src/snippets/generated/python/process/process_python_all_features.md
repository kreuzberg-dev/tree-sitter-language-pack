```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'import os\nfrom pathlib import Path\n\n# Configuration\nMY_CONST = 42\n\ndef process_file(path):\n    """Process a file and return contents."""\n    with open(path) as f:\n        return f.read()\n\nclass FileProcessor:\n    def __init__(self, base_dir):\n        self.base_dir = base_dir\n'
    config = {"comments": True, "docstrings": True, "imports": True, "language": "python", "structure": True, "symbols": True}
    _ = process(source, config)

main()

```
