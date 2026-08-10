```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "elisp"}
result = TreeSitterLanguagePack.process("(defun hello () (message \"hello\"))", config_value)

```
