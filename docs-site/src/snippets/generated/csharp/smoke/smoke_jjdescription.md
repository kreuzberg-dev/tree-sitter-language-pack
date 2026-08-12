---
id: fixture_csharp_smoke_jjdescription
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("commit message\n", new ProcessConfig { Language = "jjdescription" });

```
