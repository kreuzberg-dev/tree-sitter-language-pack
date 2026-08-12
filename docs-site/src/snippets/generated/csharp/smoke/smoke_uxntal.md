---
id: fixture_csharp_smoke_uxntal
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("|0100 LIT 01", new ProcessConfig { Language = "uxntal" });

```
