---
id: fixture_csharp_smoke_go
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package main", new ProcessConfig { Language = "go" });

```
