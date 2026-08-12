---
id: fixture_csharp_smoke_beancount
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("2024-01-01 open Assets:Bank USD", new ProcessConfig { Language = "beancount" });

```
