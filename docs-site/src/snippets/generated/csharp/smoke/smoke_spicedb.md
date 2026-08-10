```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("definition user {}\n", new ProcessConfig { Language = "spicedb" });

```
