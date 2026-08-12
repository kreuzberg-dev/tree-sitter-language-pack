---
id: fixture_csharp_data_extraction_editorconfig_section
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[*.rs]\nindent_style = space\nindent_size = 4\n", new ProcessConfig { DataExtraction = true, Language = "editorconfig" });

```
