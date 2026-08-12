---
id: fixture_csharp_smoke_smithy
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("namespace example\nstring MyString", new ProcessConfig { Language = "smithy" });

```
