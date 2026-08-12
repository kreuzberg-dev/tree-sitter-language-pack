---
id: fixture_csharp_smoke_magik
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("_method object.hello\n_endmethod", new ProcessConfig { Language = "magik" });

```
