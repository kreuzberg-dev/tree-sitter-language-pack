---
id: fixture_csharp_parsing_rust_function
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn main() {}", new ProcessConfig { Language = "rust" });

```
