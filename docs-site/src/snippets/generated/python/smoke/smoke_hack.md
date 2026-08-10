```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<?hh\nfunction main(): void {}"
    config = {"language": "hack"}
    _ = process(source, config)

main()

```
