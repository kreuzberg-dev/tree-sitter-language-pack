---
id: fixture_csharp_parsing_javascript_variable
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("const x = 1;", new ProcessConfig { Language = "javascript" });

```
