---
id: fixture_csharp_smoke_gitignore
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("*.o\n*.log", new ProcessConfig { Language = "gitignore" });

```
