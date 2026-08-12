---
id: fixture_csharp_smoke_po
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("msgid \"hello\"\nmsgstr \"world\"", new ProcessConfig { Language = "po" });

```
