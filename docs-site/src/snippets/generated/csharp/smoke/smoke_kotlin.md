---
id: fixture_csharp_smoke_kotlin
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fun main() {}", new ProcessConfig { Language = "kotlin" });

```
