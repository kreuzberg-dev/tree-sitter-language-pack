---
id: fixture_csharp_smoke_vb
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Module Main\nEnd Module", new ProcessConfig { Language = "vb" });

```
