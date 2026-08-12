---
id: fixture_csharp_smoke_pymanifest
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("include *.txt", new ProcessConfig { Language = "pymanifest" });

```
