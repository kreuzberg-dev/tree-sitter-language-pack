```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dart\"}")
_ = try TreeSitterLanguagePack.process(source: "void main() {}", config: configObj)

```
