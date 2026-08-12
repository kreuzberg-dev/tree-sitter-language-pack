---
id: fixture_csharp_smoke_vento
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("hello\n", new ProcessConfig { Language = "vento" });

```
