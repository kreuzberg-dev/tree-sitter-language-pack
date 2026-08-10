```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vala\"}")
_ = try TreeSitterLanguagePack.process(source: "class Foo {\n}\n", config: configObj)

```
