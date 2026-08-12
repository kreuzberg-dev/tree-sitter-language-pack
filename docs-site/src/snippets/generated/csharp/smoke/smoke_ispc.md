---
id: fixture_csharp_smoke_ispc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export void main() {}", new ProcessConfig { Language = "ispc" });

```
