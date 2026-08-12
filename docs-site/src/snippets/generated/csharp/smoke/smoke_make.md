---
id: fixture_csharp_smoke_make
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("all:\n\techo hello", new ProcessConfig { Language = "make" });

```
