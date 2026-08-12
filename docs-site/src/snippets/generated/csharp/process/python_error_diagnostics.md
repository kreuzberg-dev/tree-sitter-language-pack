---
id: fixture_csharp_python_error_diagnostics
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def broken(\n    pass\n", new ProcessConfig { Diagnostics = true, Language = "python" });

```
