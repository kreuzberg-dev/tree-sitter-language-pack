```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("struct Point { x: f64, y: f64 }", new ProcessConfig { Language = "rust" });

```
