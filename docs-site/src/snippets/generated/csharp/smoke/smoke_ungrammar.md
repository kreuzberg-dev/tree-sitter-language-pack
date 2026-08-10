```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Root = Item*\nItem = 'token'", new ProcessConfig { Language = "ungrammar" });

```
