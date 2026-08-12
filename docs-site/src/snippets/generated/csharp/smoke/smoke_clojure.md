---
id: fixture_csharp_smoke_clojure
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(def x 1)", new ProcessConfig { Language = "clojure" });

```
