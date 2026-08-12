---
id: fixture_csharp_smoke_llvm_mir
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---\nname: foo\n...\n", new ProcessConfig { Language = "llvm_mir" });

```
