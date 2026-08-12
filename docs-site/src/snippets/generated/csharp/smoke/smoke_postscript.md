---
id: fixture_csharp_smoke_postscript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/hello { (Hello) show } def", new ProcessConfig { Language = "postscript" });

```
