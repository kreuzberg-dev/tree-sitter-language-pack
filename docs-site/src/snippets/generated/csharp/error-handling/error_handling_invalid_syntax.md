---
id: fixture_csharp_error_handling_invalid_syntax
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function function function @@@ %%%", new ProcessConfig { Language = "javascript" });

```
