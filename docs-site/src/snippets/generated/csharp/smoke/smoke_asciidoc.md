---
id: fixture_csharp_smoke_asciidoc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("= Title\n\nParagraph.", new ProcessConfig { Language = "asciidoc" });

```
