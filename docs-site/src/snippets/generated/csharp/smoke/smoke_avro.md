---
id: fixture_csharp_smoke_avro
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("protocol P {\n}\n", new ProcessConfig { Language = "avro" });

```
