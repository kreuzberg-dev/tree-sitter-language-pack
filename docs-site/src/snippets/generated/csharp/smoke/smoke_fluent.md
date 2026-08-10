```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("hello = Hello\n", new ProcessConfig { Language = "fluent" });

```
