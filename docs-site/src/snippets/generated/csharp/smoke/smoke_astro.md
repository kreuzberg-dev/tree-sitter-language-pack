---
id: fixture_csharp_smoke_astro
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---\n---\n<p>hello</p>", new ProcessConfig { Language = "astro" });

```
