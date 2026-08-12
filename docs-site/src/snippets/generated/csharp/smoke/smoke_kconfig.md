---
id: fixture_csharp_smoke_kconfig
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("config FOO\n\tbool \"Enable foo\"", new ProcessConfig { Language = "kconfig" });

```
