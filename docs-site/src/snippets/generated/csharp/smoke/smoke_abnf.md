---
id: fixture_csharp_smoke_abnf
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("a = \"b\"\r\n", new ProcessConfig { Language = "abnf" });

```
