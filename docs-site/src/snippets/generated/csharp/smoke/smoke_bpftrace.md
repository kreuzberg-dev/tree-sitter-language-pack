---
id: fixture_csharp_smoke_bpftrace
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("BEGIN { }\n", new ProcessConfig { Language = "bpftrace" });

```
