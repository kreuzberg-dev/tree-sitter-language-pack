```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("cmake_minimum_required(VERSION 3.0)", new ProcessConfig { Language = "cmake" });

```
