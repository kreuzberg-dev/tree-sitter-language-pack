```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/** @brief A function */", new ProcessConfig { Language = "doxygen" });

```
