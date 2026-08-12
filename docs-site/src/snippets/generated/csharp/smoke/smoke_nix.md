---
id: fixture_csharp_smoke_nix
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{ pkgs ? import <nixpkgs> {} }: pkgs.hello", new ProcessConfig { Language = "nix" });

```
