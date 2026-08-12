---
id: fixture_csharp_smoke_prolog
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("hello :- write('hello'), nl.", new ProcessConfig { Language = "prolog" });

```
