```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("notify { 'hello': }", new ProcessConfig { Language = "puppet" });

```
