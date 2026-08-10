```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "latex"}
result = TreeSitterLanguagePack.process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", config_value)

```
