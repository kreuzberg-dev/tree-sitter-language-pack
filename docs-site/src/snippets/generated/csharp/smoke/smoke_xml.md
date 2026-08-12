---
id: fixture_csharp_smoke_xml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<?xml version=\"1.0\"?>\n<root>hello</root>", new ProcessConfig { Language = "xml" });

```
