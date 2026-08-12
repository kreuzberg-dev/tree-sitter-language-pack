---
id: fixture_csharp_smoke_linkerscript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SECTIONS { .text : { *(.text) } }", new ProcessConfig { Language = "linkerscript" });

```
