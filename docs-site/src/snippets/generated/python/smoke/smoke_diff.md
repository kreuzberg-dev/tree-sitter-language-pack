```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new"
    config = {"language": "diff"}
    _ = process(source, config)

main()

```
