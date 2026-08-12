---
id: fixture_csharp_smoke_powershell
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Write-Host 'hello'", new ProcessConfig { Language = "powershell" });

```
