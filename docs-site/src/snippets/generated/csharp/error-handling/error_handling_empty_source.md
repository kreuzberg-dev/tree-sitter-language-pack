---
id: fixture_csharp_error_handling_empty_source
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("", new ProcessConfig { Language = "javascript" });

```
