---
id: fixture_csharp_data_extraction_kdl_nested
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("server {\n  host \"localhost\"\n  port 8080\n}\n", new ProcessConfig { DataExtraction = true, Language = "kdl" });

```
