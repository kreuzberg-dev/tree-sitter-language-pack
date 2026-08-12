---
id: fixture_csharp_smoke_rust
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
