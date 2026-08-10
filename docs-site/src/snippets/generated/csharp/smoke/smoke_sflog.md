```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("37.0 APEX_CODE,DEBUG\n16:06:58.18 (1)|EXECUTION_STARTED\n", new ProcessConfig { Language = "sflog" });

```
