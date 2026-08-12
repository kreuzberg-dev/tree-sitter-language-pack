---
id: fixture_csharp_smoke_cobol
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", new ProcessConfig { Language = "cobol" });

```
