---
id: fixture_csharp_data_extraction_json_array
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[1, 2, 3]", new ProcessConfig { DataExtraction = true, Language = "json" });

```
