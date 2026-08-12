---
id: fixture_csharp_smoke_netlinx
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("PROGRAM_NAME='hello'", new ProcessConfig { Language = "netlinx" });

```
