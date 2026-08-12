---
id: fixture_csharp_smoke_awk
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("BEGIN { print \"hello\" }", new ProcessConfig { Language = "awk" });

```
