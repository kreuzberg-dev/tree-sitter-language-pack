---
id: fixture_csharp_data_extraction_ini_section
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[database]\nhost=localhost\nport=5432\n", new ProcessConfig { DataExtraction = true, Language = "ini" });

```
