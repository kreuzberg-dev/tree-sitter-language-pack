---
id: fixture_csharp_smoke_jq
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process(".[] | select(.key)", new ProcessConfig { Language = "jq" });

```
