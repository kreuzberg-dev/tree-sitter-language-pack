```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("table Foo {}\n", new ProcessConfig { Language = "flatbuffers" });

```
