```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(fn hello [] (print :hello))", new ProcessConfig { Language = "fennel" });

```
