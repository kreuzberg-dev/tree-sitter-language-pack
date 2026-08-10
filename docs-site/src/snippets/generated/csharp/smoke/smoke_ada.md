```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("procedure Main is begin null; end Main;", new ProcessConfig { Language = "ada" });

```
