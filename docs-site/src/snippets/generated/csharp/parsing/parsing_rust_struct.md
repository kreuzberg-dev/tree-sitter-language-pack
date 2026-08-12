---
id: fixture_csharp_parsing_rust_struct
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("struct Point { x: f64, y: f64 }", new ProcessConfig { Language = "rust" });

```
