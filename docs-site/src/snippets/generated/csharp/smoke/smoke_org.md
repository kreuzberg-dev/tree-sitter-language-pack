---
id: fixture_csharp_smoke_org
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("* Hello\nWorld", new ProcessConfig { Language = "org" });

```
