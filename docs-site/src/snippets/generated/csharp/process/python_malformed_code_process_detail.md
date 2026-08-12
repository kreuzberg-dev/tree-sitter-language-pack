---
id: fixture_csharp_python_malformed_code_process_detail
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def broken(\n    return\nclass", new ProcessConfig { Diagnostics = true, Language = "python" });

```
