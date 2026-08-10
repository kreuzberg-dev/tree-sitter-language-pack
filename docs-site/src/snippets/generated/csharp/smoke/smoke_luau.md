```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("local x: number = 1", new ProcessConfig { Language = "luau" });

```
