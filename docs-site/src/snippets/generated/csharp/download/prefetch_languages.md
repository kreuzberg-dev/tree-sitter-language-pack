```csharp title="C#"
using System.Text.Json;
using TreeSitterLanguagePack;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
TreeSitterLanguagePackConverter.Prefetch(new List<String>() { JsonSerializer.Deserialize<String>("\"python\"", ConfigOptions)! });

```
