---
id: fixture_csharp_smoke_promela
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("init {\n}\n", new ProcessConfig { Language = "promela" });

```
