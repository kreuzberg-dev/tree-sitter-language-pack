---
id: fixture_csharp_data_extraction_xml_nested
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<config><host>localhost</host><port>8080</port></config>", new ProcessConfig { DataExtraction = true, Language = "xml" });

```
