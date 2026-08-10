```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("KEY=value\n", new ProcessConfig { Language = "dotenv" });

```
