```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%p hello\n", new ProcessConfig { Language = "haml" });

```
