---
id: fixture_csharp_smoke_flatbuffers
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("table Foo {}\n", new ProcessConfig { Language = "flatbuffers" });

```
