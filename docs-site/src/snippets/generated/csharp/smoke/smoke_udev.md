---
id: fixture_csharp_smoke_udev
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("ACTION==\"add\", KERNEL==\"sd*\"", new ProcessConfig { Language = "udev" });

```
