---
id: fixture_csharp_smoke_prisma
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("model User { id Int @id }", new ProcessConfig { Language = "prisma" });

```
