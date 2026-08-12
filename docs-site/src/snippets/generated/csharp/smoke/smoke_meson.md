---
id: fixture_csharp_smoke_meson
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("project('hello', 'c')", new ProcessConfig { Language = "meson" });

```
