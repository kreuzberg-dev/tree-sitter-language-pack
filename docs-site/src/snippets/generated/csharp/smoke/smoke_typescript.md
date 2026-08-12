---
id: fixture_csharp_smoke_typescript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("const x: number = 42;", new ProcessConfig { Language = "typescript" });

```
