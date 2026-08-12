---
id: fixture_csharp_smoke_matlab
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function y = hello(x)\ny = x;\nend", new ProcessConfig { Language = "matlab" });

```
