```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def first():\n    x = 1\n    return x\n\ndef second():\n    y = 2\n    return y\n\ndef third():\n    z = 3\n    return z\n", new ProcessConfig { ChunkMaxSize = 50, Language = "python" });

```
