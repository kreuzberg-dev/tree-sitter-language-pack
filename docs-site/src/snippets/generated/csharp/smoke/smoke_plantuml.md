```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@startuml\n@enduml\n", new ProcessConfig { Language = "plantuml" });

```
