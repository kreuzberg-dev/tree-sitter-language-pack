---
id: fixture_csharp_smoke_proto
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("syntax = \"proto3\";", new ProcessConfig { Language = "proto" });

```
