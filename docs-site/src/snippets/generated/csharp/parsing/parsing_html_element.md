---
id: fixture_csharp_parsing_html_element
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<div>hello</div>", new ProcessConfig { Language = "html" });

```
