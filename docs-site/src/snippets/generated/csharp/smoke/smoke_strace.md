```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("open(\"/x\", O_RDONLY) = 3\n", new ProcessConfig { Language = "strace" });

```
