```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def hello(): pass", new ProcessConfig { Language = "python" });

```
