---
id: fixture_csharp_smoke_ada
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("procedure Main is begin null; end Main;", new ProcessConfig { Language = "ada" });

```
