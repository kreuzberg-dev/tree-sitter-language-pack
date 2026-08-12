---
id: fixture_csharp_smoke_yuck
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(defwidget main [] (label :text \"hi\"))", new ProcessConfig { Language = "yuck" });

```
