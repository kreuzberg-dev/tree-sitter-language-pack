---
id: fixture_csharp_smoke_rst
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Hello\n=====\n\nWorld", new ProcessConfig { Language = "rst" });

```
