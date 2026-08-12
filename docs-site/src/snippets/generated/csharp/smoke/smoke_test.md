---
id: fixture_csharp_smoke_test
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("===========\nTest\n===========\n---\n(node)", new ProcessConfig { Language = "test" });

```
