```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Module Main\nEnd Module", new ProcessConfig { Language = "vb" });

```
