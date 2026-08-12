---
id: fixture_csharp_smoke_arduino
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("void setup() {}", new ProcessConfig { Language = "arduino" });

```
