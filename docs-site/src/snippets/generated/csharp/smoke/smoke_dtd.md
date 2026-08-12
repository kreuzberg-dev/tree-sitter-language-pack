---
id: fixture_csharp_smoke_dtd
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<!ELEMENT note (body)>", new ProcessConfig { Language = "dtd" });

```
