```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sparql"}
result = TreeSitterLanguagePack.process("SELECT ?s WHERE { ?s ?p ?o }", config_value)

```
