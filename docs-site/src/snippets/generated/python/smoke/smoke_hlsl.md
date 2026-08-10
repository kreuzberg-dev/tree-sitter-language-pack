```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "float4 main() : SV_Target { return 0; }"
    config = {"language": "hlsl"}
    _ = process(source, config)

main()

```
