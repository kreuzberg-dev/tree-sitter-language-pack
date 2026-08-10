```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", new ProcessConfig { Language = "pem" });

```
