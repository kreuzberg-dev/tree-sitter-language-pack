---
id: fixture_csharp_smoke_haskell_persistent
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Person\n  name String\n", new ProcessConfig { Language = "haskell_persistent" });

```
