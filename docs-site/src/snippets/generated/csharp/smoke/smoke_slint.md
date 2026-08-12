---
id: fixture_csharp_smoke_slint
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export component Foo {}\n", new ProcessConfig { Language = "slint" });

```
