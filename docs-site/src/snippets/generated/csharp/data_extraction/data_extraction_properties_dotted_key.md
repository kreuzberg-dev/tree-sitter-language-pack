---
id: fixture_csharp_data_extraction_properties_dotted_key
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("server.host=localhost\nserver.port=8080\n", new ProcessConfig { DataExtraction = true, Language = "properties" });

```
