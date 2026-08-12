---
id: fixture_csharp_smoke_cedar
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("permit(principal, action, resource);", new ProcessConfig { Language = "cedar" });

```
