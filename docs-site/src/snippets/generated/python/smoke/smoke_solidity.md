```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "pragma solidity ^0.8.0;\ncontract Main {}"
    config = {"language": "solidity"}
    _ = process(source, config)

main()

```
