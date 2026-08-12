---
id: fixture_csharp_smoke_asm
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("mov eax, 1", new ProcessConfig { Language = "asm" });

```
