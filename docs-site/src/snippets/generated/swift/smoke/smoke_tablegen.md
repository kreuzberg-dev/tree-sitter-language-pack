```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tablegen\"}")
_ = try TreeSitterLanguagePack.process(source: "def Hello : Base {}", config: configObj)

```
