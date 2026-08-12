---
id: fixture_csharp_smoke_doxygen
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/** @brief A function */", new ProcessConfig { Language = "doxygen" });

```
