```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fn main() {}", new ProcessConfig { Language = "rust" });

```
