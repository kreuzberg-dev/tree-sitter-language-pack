---
id: fixture_csharp_smoke_html
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<p>hello</p>", new ProcessConfig { Language = "html" });

```
