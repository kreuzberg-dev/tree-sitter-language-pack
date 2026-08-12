---
id: fixture_csharp_parsing_javascript_class
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Foo { bar() {} }", new ProcessConfig { Language = "javascript" });

```
