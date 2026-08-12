---
id: fixture_csharp_data_extraction_json_nested
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\"server\": {\"host\": \"x\", \"port\": 8080}}", new ProcessConfig { DataExtraction = true, Language = "json" });

```
