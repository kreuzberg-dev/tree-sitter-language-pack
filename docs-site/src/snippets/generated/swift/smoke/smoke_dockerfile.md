```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dockerfile\"}")
_ = try TreeSitterLanguagePack.process(source: "FROM alpine", config: configObj)

```
