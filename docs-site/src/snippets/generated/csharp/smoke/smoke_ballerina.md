```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("public function main() {\n}\n", new ProcessConfig { Language = "ballerina" });

```
