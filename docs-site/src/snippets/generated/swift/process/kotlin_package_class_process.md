```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kotlin\"}")
_ = try TreeSitterLanguagePack.process(source: "package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", config: configObj)

```
