```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Foo {\n}\n", new ProcessConfig { Language = "vala" });

```
