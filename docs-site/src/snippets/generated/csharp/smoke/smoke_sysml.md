```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package P {}\n", new ProcessConfig { Language = "sysml" });

```
