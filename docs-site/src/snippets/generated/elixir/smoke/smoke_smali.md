```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "smali"}
result = TreeSitterLanguagePack.process(".class public LMain;\n.super Ljava/lang/Object;", config_value)

```
