---
id: fixture_csharp_smoke_squirrel
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function main() {}", new ProcessConfig { Language = "squirrel" });

```
