```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("() recv_internal() {}", new ProcessConfig { Language = "func" });

```
