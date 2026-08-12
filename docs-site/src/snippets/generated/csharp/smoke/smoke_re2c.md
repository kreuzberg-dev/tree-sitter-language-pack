---
id: fixture_csharp_smoke_re2c
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/*!re2c\n  [a-z]+ { return; }\n*/", new ProcessConfig { Language = "re2c" });

```
