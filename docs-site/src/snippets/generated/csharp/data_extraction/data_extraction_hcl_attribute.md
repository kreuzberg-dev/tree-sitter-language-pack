---
id: fixture_csharp_data_extraction_hcl_attribute
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("region = \"us-east-1\"\ncount  = 3\n", new ProcessConfig { DataExtraction = true, Language = "hcl" });

```
