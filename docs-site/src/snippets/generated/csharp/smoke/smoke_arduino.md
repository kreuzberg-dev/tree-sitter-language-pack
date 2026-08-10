```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("void setup() {}", new ProcessConfig { Language = "arduino" });

```
