---
id: fixture_csharp_smoke_rshtml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<p>hi</p>\n", new ProcessConfig { Language = "rshtml" });

```
