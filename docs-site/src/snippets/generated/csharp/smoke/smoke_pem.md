---
id: fixture_csharp_smoke_pem
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", new ProcessConfig { Language = "pem" });

```
