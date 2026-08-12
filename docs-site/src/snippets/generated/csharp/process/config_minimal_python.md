---
id: fixture_csharp_config_minimal_python
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def hello():\n    pass\n", new ProcessConfig { Language = "python" });

```
