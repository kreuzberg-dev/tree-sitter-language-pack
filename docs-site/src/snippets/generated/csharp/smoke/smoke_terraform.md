---
id: fixture_csharp_smoke_terraform
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("resource \"null_resource\" \"main\" {}", new ProcessConfig { Language = "terraform" });

```
