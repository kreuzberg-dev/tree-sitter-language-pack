---
id: fixture_csharp_smoke_haskell
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("main = putStrLn \"hello\"", new ProcessConfig { Language = "haskell" });

```
