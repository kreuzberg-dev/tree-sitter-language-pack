---
id: fixture_csharp_smoke_motoko
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("actor {\n}\n", new ProcessConfig { Language = "motoko" });

```
