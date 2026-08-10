```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"c\"}")
_ = try TreeSitterLanguagePack.process(source: "#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", config: configObj)

```
