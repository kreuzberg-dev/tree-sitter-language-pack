---
id: fixture_csharp_smoke_javascript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("console.log('hello');", new ProcessConfig { Language = "javascript" });

```
