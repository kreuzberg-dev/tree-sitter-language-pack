---
id: fixture_csharp_smoke_pony
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("actor Main\n  new create(env: Env) => None", new ProcessConfig { Language = "pony" });

```
