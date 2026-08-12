---
id: fixture_csharp_smoke_yul
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("object \"C\" {\n  code {\n  }\n}\n", new ProcessConfig { Language = "yul" });

```
