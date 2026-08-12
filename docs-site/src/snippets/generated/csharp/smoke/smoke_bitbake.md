---
id: fixture_csharp_smoke_bitbake
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("DESCRIPTION = \"hello\"", new ProcessConfig { Language = "bitbake" });

```
