---
id: fixture_csharp_smoke_fortran
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("program main\nend program main", new ProcessConfig { Language = "fortran" });

```
