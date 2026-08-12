---
id: fixture_csharp_smoke_gleam
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("pub fn main() { }", new ProcessConfig { Language = "gleam" });

```
