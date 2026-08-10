```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process(".class public LMain;\n.super Ljava/lang/Object;", new ProcessConfig { Language = "smali" });

```
