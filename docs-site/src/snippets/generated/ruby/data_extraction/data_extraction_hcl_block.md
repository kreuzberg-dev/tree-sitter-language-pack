```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("resource \"aws_instance\" \"web\" {\n  ami = \"ami-123\"\n  instance_type = \"t2.micro\"\n}\n", { 'data_extraction' => true, 'language' => 'hcl' })

```
