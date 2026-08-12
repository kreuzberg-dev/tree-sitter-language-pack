---
id: fixture_csharp_smoke_mermaid
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("graph TD\nA --> B", new ProcessConfig { Language = "mermaid" });

```
