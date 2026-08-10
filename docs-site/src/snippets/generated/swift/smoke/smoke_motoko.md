```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"motoko\"}")
_ = try TreeSitterLanguagePack.process(source: "actor {\n}\n", config: configObj)

```
