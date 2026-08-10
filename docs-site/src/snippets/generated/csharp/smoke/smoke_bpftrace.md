```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("BEGIN { }\n", new ProcessConfig { Language = "bpftrace" });

```
