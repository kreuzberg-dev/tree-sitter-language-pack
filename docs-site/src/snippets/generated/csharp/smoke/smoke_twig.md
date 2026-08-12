---
id: fixture_csharp_smoke_twig
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{{ variable }}", new ProcessConfig { Language = "twig" });

```
