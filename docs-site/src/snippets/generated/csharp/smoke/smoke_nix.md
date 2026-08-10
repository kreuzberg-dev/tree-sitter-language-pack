```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{ pkgs ? import <nixpkgs> {} }: pkgs.hello", new ProcessConfig { Language = "nix" });

```
