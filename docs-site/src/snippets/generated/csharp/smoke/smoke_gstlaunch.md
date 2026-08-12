---
id: fixture_csharp_smoke_gstlaunch
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fakesrc ! fakesink", new ProcessConfig { Language = "gstlaunch" });

```
