```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("export component Foo {}\n", new ProcessConfig { Language = "slint" });

```
