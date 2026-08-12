---
id: fixture_csharp_smoke_cpon
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\"key\": 1}", new ProcessConfig { Language = "cpon" });

```
