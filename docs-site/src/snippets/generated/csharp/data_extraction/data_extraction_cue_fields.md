---
id: fixture_csharp_data_extraction_cue_fields
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("host: \"localhost\"\nport: 8080\n", new ProcessConfig { DataExtraction = true, Language = "cue" });

```
