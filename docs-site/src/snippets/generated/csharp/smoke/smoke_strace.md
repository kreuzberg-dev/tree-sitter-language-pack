---
id: fixture_csharp_smoke_strace
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("open(\"/x\", O_RDONLY) = 3\n", new ProcessConfig { Language = "strace" });

```
