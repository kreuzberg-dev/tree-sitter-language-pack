---
id: fixture_csharp_smoke_tablegen
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def Hello : Base {}", new ProcessConfig { Language = "tablegen" });

```
