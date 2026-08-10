```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/hello { (Hello) show } def", new ProcessConfig { Language = "postscript" });

```
