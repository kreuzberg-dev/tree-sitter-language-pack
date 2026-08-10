```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"corn\"}")
_ = try TreeSitterLanguagePack.process(source: "{ key = \"value\" }", config: configObj)

```
