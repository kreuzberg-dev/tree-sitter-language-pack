---
id: fixture_csharp_smoke_zsh
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("echo hello", new ProcessConfig { Language = "zsh" });

```
