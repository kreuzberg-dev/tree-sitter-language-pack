---
id: fixture_csharp_data_extraction_json_flat
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\"host\": \"localhost\", \"port\": 8080}", new ProcessConfig { DataExtraction = true, Language = "json" });

```
