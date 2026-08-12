---
id: fixture_csharp_smoke_dotenv
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("KEY=value\n", new ProcessConfig { Language = "dotenv" });

```
