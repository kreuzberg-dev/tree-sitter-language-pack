---
id: fixture_csharp_data_extraction_caddy_directives
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("localhost\nroot * /var/www\nfile_server\n", new ProcessConfig { DataExtraction = true, Language = "caddy" });

```
