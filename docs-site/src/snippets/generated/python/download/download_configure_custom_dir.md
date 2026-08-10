```python title="Python"
from tree_sitter_language_pack import configure

def main() -> None:
    config = {"cache_dir": "/tmp/tslp_test_cache"}  # noqa: S108
    _ = configure(config)

main()

```
