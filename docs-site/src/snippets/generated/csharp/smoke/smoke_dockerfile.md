---
id: fixture_csharp_smoke_dockerfile
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("FROM alpine", new ProcessConfig { Language = "dockerfile" });

```
