---
id: fixture_csharp_smoke_batch
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@echo off\necho hello", new ProcessConfig { Language = "batch" });

```
