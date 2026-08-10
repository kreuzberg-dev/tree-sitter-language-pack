```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("service HelloService {}", new ProcessConfig { Language = "thrift" });

```
