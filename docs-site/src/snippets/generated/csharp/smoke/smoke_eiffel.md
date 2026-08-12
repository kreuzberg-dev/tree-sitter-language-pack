---
id: fixture_csharp_smoke_eiffel
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class FOO\nend\n", new ProcessConfig { Language = "eiffel" });

```
