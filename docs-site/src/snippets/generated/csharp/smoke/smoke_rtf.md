---
id: fixture_csharp_smoke_rtf
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\\rtf1 hello}", new ProcessConfig { Language = "rtf" });

```
