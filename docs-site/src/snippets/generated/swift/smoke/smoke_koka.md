```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"koka\"}")
_ = try TreeSitterLanguagePack.process(source: "fun main()\n  1\n", config: configObj)

```
