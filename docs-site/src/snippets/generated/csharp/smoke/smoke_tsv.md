---
id: fixture_csharp_smoke_tsv
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("a\tb\tc\n1\t2\t3", new ProcessConfig { Language = "tsv" });

```
