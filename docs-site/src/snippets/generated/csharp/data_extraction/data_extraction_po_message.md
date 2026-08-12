---
id: fixture_csharp_data_extraction_po_message
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", new ProcessConfig { DataExtraction = true, Language = "po" });

```
