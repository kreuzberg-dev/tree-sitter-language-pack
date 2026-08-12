---
id: fixture_csharp_smoke_java
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Main {}", new ProcessConfig { Language = "java" });

```
