---
id: fixture_csharp_smoke_haml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%p hello\n", new ProcessConfig { Language = "haml" });

```
