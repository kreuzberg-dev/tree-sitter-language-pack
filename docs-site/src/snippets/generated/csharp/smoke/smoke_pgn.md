---
id: fixture_csharp_smoke_pgn
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("1. e4 e5 *", new ProcessConfig { Language = "pgn" });

```
