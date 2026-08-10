```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"java\"}")
_ = try TreeSitterLanguagePack.process(source: "import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return \"Hello \" + name;\n    }\n}\n", config: configObj)

```
