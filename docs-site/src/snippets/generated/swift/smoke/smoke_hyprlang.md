```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hyprlang\"}")
_ = try TreeSitterLanguagePack.process(source: "general { border_size = 1 }", config: configObj)

```
