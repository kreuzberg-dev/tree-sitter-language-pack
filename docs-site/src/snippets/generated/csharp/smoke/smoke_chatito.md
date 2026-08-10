```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%[greeting]\n    hello", new ProcessConfig { Language = "chatito" });

```
