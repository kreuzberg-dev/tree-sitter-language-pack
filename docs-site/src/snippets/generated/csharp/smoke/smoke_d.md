---
id: fixture_csharp_smoke_d
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("void main() {}", new ProcessConfig { Language = "d" });

```
