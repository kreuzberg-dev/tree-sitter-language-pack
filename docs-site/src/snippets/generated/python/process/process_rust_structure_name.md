```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "pub struct MyConfig {\n    pub name: String,\n    pub value: i32,\n}\n\nimpl MyConfig {\n    pub fn new() -> Self {\n        Self { name: String::new(), value: 0 }\n    }\n}\n"
    config = {"language": "rust"}
    _ = process(source, config)

main()

```
