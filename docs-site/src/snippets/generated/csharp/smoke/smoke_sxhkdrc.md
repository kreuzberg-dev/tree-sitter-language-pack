```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("super + a\n\techo hi\n", new ProcessConfig { Language = "sxhkdrc" });

```
