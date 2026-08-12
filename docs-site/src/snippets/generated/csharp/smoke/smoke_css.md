---
id: fixture_csharp_smoke_css
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("body { color: red; }", new ProcessConfig { Language = "css" });

```
