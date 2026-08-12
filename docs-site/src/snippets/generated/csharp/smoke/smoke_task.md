---
id: fixture_csharp_smoke_task
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("todo item\n", new ProcessConfig { Language = "task" });

```
