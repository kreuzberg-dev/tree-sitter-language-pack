---
id: fixture_csharp_smoke_heex
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<%= @greeting %>", new ProcessConfig { Language = "heex" });

```
