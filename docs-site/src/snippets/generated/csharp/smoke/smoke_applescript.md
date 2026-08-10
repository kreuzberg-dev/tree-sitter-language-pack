```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("set x to 1\n", new ProcessConfig { Language = "applescript" });

```
