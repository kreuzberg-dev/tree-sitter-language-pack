```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'resource "aws_instance" "web" {\n  ami = "ami-123"\n  instance_type = "t2.micro"\n}\n'
    config = {"data_extraction": True, "language": "hcl"}
    _ = process(source, config)

main()

```
