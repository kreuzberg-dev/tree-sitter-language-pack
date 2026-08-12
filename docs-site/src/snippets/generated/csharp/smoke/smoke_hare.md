---
id: fixture_csharp_smoke_hare
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export fn main() void = void;", new ProcessConfig { Language = "hare" });

```
