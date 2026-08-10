```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("variable \"name\" { type = string }", new ProcessConfig { Language = "hcl" });

```
