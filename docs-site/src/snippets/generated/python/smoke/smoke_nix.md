```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "{ pkgs ? import <nixpkgs> {} }: pkgs.hello"
    config = {"language": "nix"}
    _ = process(source, config)

main()

```
