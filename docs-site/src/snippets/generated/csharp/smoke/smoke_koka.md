---
id: fixture_csharp_smoke_koka
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fun main()\n  1\n", new ProcessConfig { Language = "koka" });

```
