---
id: fixture_csharp_smoke_markdown
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("# Hello\n\nWorld", new ProcessConfig { Language = "markdown" });

```
