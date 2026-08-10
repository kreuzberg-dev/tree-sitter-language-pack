```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"eiffel\"}")
_ = try TreeSitterLanguagePack.process(source: "class FOO\nend\n", config: configObj)

```
