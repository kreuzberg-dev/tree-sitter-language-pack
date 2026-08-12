---
id: fixture_csharp_smoke_func
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("() recv_internal() {}", new ProcessConfig { Language = "func" });

```
