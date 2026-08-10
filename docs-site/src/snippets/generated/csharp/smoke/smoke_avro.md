```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("protocol P {\n}\n", new ProcessConfig { Language = "avro" });

```
