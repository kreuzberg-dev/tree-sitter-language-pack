```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("all:\n\techo hello", new ProcessConfig { Language = "make" });

```
