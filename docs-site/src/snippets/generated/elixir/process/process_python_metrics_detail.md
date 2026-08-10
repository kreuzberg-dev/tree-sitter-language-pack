```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("\# module docstring\nimport os\n\ndef hello():\n    \# greeting\n    print('hello')\n\ndef world():\n    print('world')\n", config_value)

```
