---
id: fixture_csharp_data_extraction_dtd_element_decl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", new ProcessConfig { DataExtraction = true, Language = "dtd" });

```
