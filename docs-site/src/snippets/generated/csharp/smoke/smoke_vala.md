---
id: fixture_csharp_smoke_vala
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Foo {\n}\n", new ProcessConfig { Language = "vala" });

```
