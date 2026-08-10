```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module main; endmodule"
    config = {"language": "verilog"}
    _ = process(source, config)

main()

```
