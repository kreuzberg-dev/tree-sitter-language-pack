```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("main => true.\n", new ProcessConfig { Language = "picat" });

```
