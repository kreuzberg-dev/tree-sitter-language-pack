---
id: fixture_csharp_smoke_applescript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("set x to 1\n", new ProcessConfig { Language = "applescript" });

```
