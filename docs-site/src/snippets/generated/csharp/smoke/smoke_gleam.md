```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("pub fn main() { }", new ProcessConfig { Language = "gleam" });

```
