---
id: fixture_csharp_data_extraction_toml_array
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("ports = [8080, 8081, 8082]\n", new ProcessConfig { DataExtraction = true, Language = "toml" });

```
