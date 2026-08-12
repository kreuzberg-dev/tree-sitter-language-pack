---
id: fixture_csharp_smoke_toml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("key = \"value\"", new ProcessConfig { Language = "toml" });

```
