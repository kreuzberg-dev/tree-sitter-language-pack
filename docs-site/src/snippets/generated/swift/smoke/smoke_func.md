```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"func\"}")
_ = try TreeSitterLanguagePack.process(source: "() recv_internal() {}", config: configObj)

```
