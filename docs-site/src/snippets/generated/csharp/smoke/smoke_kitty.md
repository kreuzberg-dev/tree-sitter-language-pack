```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("font_size 12\n", new ProcessConfig { Language = "kitty" });

```
