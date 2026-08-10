```python title="Python"
from tree_sitter_language_pack import detect_language_from_content

def main() -> None:
    content = "#!/bin/bash\necho hi"
    _ = detect_language_from_content(content)

main()

```
