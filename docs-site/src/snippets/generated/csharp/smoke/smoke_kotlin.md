```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("fun main() {}", new ProcessConfig { Language = "kotlin" });

```
