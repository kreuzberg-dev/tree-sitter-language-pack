---
id: fixture_csharp_smoke_sysml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package P {}\n", new ProcessConfig { Language = "sysml" });

```
