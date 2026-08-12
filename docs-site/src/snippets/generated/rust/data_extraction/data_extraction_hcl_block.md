---
id: fixture_rust_data_extraction_hcl_block
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"resource "aws_instance" "web" {
      ami = "ami-123"
      instance_type = "t2.micro"
}

```
