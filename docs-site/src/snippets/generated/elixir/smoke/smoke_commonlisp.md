```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "commonlisp"}
result = TreeSitterLanguagePack.process("(defun hello () (print \"hello\"))", config_value)

```
