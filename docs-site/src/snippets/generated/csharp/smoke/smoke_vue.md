```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<template><div>hello</div></template>", new ProcessConfig { Language = "vue" });

```
