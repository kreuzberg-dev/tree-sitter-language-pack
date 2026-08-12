---
id: fixture_csharp_data_extraction_nginx_directives
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", new ProcessConfig { DataExtraction = true, Language = "nginx" });

```
