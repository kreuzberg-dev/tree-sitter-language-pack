---
id: fixture_csharp_data_extraction_yaml_sequence
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("ports:\n  - 8080\n  - 8081\n", new ProcessConfig { DataExtraction = true, Language = "yaml" });

```
