```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("__global__ void kernel() {}", new ProcessConfig { Language = "cuda" });

```
