```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("namespace example\nstring MyString", new ProcessConfig { Language = "smithy" });

```
