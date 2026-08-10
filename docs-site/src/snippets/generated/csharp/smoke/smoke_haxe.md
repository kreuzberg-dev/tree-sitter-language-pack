```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("class Main { static function main() {} }", new ProcessConfig { Language = "haxe" });

```
