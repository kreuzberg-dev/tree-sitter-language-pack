---
id: fixture_csharp_smoke_puppet
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("notify { 'hello': }", new ProcessConfig { Language = "puppet" });

```
