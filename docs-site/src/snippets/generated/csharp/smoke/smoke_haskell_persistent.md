```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Person\n  name String\n", new ProcessConfig { Language = "haskell_persistent" });

```
