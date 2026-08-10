```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ruby"}
result = TreeSitterLanguagePack.process("require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello \#{name}\"\n  end\nend\n", config_value)

```
