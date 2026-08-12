---
id: fixture_csharp_data_extraction_json_empty_object
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{}", new ProcessConfig { DataExtraction = true, Language = "json" });

```
