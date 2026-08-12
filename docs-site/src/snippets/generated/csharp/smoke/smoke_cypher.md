---
id: fixture_csharp_smoke_cypher
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("MATCH (n) RETURN n\n", new ProcessConfig { Language = "cypher" });

```
