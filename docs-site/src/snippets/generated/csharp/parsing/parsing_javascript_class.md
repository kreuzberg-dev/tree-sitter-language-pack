```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Foo { bar() {} }", new ProcessConfig { Language = "javascript" });

```
