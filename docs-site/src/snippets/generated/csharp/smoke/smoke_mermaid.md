```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("graph TD\nA --> B", new ProcessConfig { Language = "mermaid" });

```
