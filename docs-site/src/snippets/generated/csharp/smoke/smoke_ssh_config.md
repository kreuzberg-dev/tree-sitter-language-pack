---
id: fixture_csharp_smoke_ssh_config
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Host example\n  HostName example.com", new ProcessConfig { Language = "ssh_config" });

```
