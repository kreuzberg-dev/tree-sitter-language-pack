```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"unison\"}")
_ = try TreeSitterLanguagePack.process(source: "x = 1\n", config: configObj)

```
