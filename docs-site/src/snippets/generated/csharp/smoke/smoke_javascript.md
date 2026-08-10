```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("console.log('hello');", new ProcessConfig { Language = "javascript" });

```
