---
id: fixture_csharp_smoke_chatito
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%[greeting]\n    hello", new ProcessConfig { Language = "chatito" });

```
