```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"godot_resource\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
