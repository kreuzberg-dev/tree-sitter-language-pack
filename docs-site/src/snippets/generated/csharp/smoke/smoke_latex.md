---
id: fixture_csharp_smoke_latex
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", new ProcessConfig { Language = "latex" });

```
