```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", new ProcessConfig { Language = "wgsl" });

```
