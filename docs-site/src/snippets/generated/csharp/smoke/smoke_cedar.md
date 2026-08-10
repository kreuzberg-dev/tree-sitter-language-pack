```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("permit(principal, action, resource);", new ProcessConfig { Language = "cedar" });

```
