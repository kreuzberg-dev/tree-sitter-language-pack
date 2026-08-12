---
id: fixture_csharp_smoke_ocaml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("let () = print_endline \"hello\"", new ProcessConfig { Language = "ocaml" });

```
