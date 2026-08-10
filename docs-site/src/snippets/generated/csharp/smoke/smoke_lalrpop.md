```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("grammar;\n", new ProcessConfig { Language = "lalrpop" });

```
