```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("float4 main() : SV_Target { return 0; }", new ProcessConfig { Language = "hlsl" });

```
