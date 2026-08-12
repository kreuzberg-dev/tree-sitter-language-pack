---
id: fixture_csharp_smoke_aiken
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn main() {\n  1\n}\n", new ProcessConfig { Language = "aiken" });

```
