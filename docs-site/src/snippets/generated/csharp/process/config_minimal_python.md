```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def hello():\n    pass\n", new ProcessConfig { Language = "python" });

```
