---
id: fixture_csharp_smoke_xresources
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("*.foreground: #ffffff\n", new ProcessConfig { Language = "xresources" });

```
