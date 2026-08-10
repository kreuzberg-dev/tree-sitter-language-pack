```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("pub fn main() void {}", new ProcessConfig { Language = "zig" });

```
