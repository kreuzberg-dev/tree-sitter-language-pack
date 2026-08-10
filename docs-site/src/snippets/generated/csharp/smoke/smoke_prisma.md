```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("model User { id Int @id }", new ProcessConfig { Language = "prisma" });

```
