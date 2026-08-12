---
id: fixture_csharp_smoke_idl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module M {\n};\n", new ProcessConfig { Language = "idl" });

```
