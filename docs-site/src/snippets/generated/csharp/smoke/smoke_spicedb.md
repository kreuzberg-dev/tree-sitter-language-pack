---
id: fixture_csharp_smoke_spicedb
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("definition user {}\n", new ProcessConfig { Language = "spicedb" });

```
