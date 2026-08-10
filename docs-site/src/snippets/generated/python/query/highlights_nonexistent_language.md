```python title="Python"
from tree_sitter_language_pack import get_highlights_query

def main() -> None:
    language = "zzz_nonexistent_lang"
    _ = get_highlights_query(language)

main()

```
