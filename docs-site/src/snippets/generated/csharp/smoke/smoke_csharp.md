---
id: fixture_csharp_smoke_csharp
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Main {}", new ProcessConfig { Language = "csharp" });

```
