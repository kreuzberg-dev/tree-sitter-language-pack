---
id: fixture_csharp_smoke_julia
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function main() end", new ProcessConfig { Language = "julia" });

```
