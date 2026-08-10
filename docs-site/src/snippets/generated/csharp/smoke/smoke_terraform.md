```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("resource \"null_resource\" \"main\" {}", new ProcessConfig { Language = "terraform" });

```
