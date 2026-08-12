---
id: fixture_csharp_smoke_ocaml_interface
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("val x : int", new ProcessConfig { Language = "ocaml_interface" });

```
