```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", new ProcessConfig { DataExtraction = true, Language = "nginx" });

```
