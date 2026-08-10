```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("define i32 @main() { ret i32 0 }", new ProcessConfig { Language = "llvm" });

```
