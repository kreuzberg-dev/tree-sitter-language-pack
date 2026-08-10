```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%token EOF\n%%\n", new ProcessConfig { Language = "menhir" });

```
