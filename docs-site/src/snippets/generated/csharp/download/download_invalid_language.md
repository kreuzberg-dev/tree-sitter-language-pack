```csharp title="C#"
using System.Text.Json;
using TreeSitterLanguagePack;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = TreeSitterLanguagePackConverter.Download(new List<String>() { JsonSerializer.Deserialize<String>("\"zzz_definitely_not_a_real_language_xyz\"", ConfigOptions)! });

```
