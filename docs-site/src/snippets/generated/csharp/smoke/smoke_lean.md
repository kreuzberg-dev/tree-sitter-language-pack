---
id: fixture_csharp_smoke_lean
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def main : IO Unit := pure ()", new ProcessConfig { Language = "lean" });

```
