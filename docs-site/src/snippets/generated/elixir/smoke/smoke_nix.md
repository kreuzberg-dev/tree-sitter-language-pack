```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "nix"}
result = TreeSitterLanguagePack.process("{ pkgs ? import <nixpkgs> {} }: pkgs.hello", config_value)

```
