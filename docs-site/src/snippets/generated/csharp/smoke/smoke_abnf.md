```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("a = \"b\"\r\n", new ProcessConfig { Language = "abnf" });

```
