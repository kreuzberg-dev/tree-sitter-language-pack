```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("function function function @@@ %%%", new ProcessConfig { Language = "javascript" });

```
