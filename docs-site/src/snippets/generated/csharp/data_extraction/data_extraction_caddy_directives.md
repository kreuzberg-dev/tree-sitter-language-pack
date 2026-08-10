```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("localhost\nroot * /var/www\nfile_server\n", new ProcessConfig { DataExtraction = true, Language = "caddy" });

```
