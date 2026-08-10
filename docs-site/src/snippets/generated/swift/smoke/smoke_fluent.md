```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fluent\"}")
_ = try TreeSitterLanguagePack.process(source: "hello = Hello\n", config: configObj)

```
