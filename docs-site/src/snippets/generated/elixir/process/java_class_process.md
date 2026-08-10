```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "java"}
result = TreeSitterLanguagePack.process("import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return \"Hello \" + name;\n    }\n}\n", config_value)

```
