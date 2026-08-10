```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/** @param {string} name */", new ProcessConfig { Language = "jsdoc" });

```
