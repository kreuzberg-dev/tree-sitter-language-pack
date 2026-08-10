```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"actionscript\"}")
_ = try TreeSitterLanguagePack.process(source: "var x:int = 1;", config: configObj)

```
