```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "diff"}
result = TreeSitterLanguagePack.process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", config_value)

```
