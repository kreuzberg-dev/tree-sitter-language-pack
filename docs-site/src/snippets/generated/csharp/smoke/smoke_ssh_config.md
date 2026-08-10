```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Host example\n  HostName example.com", new ProcessConfig { Language = "ssh_config" });

```
