```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Write-Host 'hello'", new ProcessConfig { Language = "powershell" });

```
