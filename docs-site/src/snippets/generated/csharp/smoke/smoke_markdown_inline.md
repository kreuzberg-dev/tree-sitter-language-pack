---
id: fixture_csharp_smoke_markdown_inline
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("**bold** and *italic*", new ProcessConfig { Language = "markdown_inline" });

```
