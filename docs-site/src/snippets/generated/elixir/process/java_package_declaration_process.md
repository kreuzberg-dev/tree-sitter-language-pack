```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "java"}
result = TreeSitterLanguagePack.process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", config_value)

```
