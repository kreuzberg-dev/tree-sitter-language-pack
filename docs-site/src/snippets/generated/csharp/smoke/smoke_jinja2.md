---
id: fixture_csharp_smoke_jinja2
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{{ variable }}", new ProcessConfig { Language = "jinja2" });

```
