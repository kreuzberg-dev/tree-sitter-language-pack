```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", new ProcessConfig { Language = "rust" });

```
