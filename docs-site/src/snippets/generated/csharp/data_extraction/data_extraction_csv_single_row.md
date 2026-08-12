---
id: fixture_csharp_data_extraction_csv_single_row
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("x,y,z\n", new ProcessConfig { DataExtraction = true, Language = "csv" });

```
