---
id: fixture_csharp_smoke_actionscript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("var x:int = 1;", new ProcessConfig { Language = "actionscript" });

```
