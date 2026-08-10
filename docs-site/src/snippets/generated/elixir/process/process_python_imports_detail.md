```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", config_value)

```
