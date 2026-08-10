```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rust"}
result = TreeSitterLanguagePack.process("pub struct MyConfig {\n    pub name: String,\n    pub value: i32,\n}\n\nimpl MyConfig {\n    pub fn new() -> Self {\n        Self { name: String::new(), value: 0 }\n    }\n}\n", config_value)

```
