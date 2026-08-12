---
id: fixture_csharp_smoke_llvm
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("define i32 @main() { ret i32 0 }", new ProcessConfig { Language = "llvm" });

```
