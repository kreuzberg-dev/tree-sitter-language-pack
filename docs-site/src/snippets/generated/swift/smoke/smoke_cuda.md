```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cuda\"}")
_ = try TreeSitterLanguagePack.process(source: "__global__ void kernel() {}", config: configObj)

```
