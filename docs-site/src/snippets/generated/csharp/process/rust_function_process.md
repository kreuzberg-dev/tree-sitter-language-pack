---
id: fixture_csharp_rust_function_process
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", new ProcessConfig { Language = "rust" });

```
