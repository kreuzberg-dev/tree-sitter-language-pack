```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("===========\nTest\n===========\n---\n(node)", new ProcessConfig { Language = "test" });

```
