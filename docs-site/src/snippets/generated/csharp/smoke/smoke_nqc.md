---
id: fixture_csharp_smoke_nqc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("task main() {}", new ProcessConfig { Language = "nqc" });

```
