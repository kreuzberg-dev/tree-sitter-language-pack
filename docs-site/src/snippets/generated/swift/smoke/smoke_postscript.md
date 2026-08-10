```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"postscript\"}")
_ = try TreeSitterLanguagePack.process(source: "/hello { (Hello) show } def", config: configObj)

```
