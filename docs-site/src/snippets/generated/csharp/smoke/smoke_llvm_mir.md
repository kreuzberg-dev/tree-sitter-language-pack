```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---\nname: foo\n...\n", new ProcessConfig { Language = "llvm_mir" });

```
