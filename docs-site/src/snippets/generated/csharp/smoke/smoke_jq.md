```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process(".[] | select(.key)", new ProcessConfig { Language = "jq" });

```
