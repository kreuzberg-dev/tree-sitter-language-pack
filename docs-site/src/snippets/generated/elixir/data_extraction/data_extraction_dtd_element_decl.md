```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "dtd"}
result = TreeSitterLanguagePack.process("<!ELEMENT server (host, port)>\n<!ELEMENT host (\#PCDATA)>\n", config_value)

```
