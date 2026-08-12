---
id: fixture_csharp_smoke_gn
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("group(\"hello\") {}", new ProcessConfig { Language = "gn" });

```
