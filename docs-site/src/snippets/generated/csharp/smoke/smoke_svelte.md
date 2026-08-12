---
id: fixture_csharp_smoke_svelte
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<script>let x = 1;</script>", new ProcessConfig { Language = "svelte" });

```
