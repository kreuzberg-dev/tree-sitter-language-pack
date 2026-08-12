---
id: fixture_csharp_data_extraction_xml_element
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<server id=\"main\"><host>localhost</host></server>", new ProcessConfig { DataExtraction = true, Language = "xml" });

```
