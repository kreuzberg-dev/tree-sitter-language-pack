```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module m {\n}\n", new ProcessConfig { Language = "yang" });

```
