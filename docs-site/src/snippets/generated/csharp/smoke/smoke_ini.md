---
id: fixture_csharp_smoke_ini
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[section]\nkey = value", new ProcessConfig { Language = "ini" });

```
