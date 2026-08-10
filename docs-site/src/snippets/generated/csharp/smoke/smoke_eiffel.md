```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class FOO\nend\n", new ProcessConfig { Language = "eiffel" });

```
