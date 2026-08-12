---
id: fixture_csharp_smoke_gitcommit
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("feat: add feature\n\nBody text", new ProcessConfig { Language = "gitcommit" });

```
