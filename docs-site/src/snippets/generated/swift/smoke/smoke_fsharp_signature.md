```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fsharp_signature\"}")
_ = try TreeSitterLanguagePack.process(source: "val x: int", config: configObj)

```
