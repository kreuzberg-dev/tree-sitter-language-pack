---
id: fixture_csharp_smoke_ungrammar
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Root = Item*\nItem = 'token'", new ProcessConfig { Language = "ungrammar" });

```
