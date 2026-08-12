---
id: fixture_csharp_smoke_embeddedtemplate
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<%= value %>", new ProcessConfig { Language = "embeddedtemplate" });

```
