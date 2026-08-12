---
id: fixture_csharp_smoke_gomod
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module example.com/hello\n\ngo 1.21", new ProcessConfig { Language = "gomod" });

```
