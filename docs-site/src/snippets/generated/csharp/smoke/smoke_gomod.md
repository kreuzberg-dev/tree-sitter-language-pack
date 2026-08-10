```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module example.com/hello\n\ngo 1.21", new ProcessConfig { Language = "gomod" });

```
