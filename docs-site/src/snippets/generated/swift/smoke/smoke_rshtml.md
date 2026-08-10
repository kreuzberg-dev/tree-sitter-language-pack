```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rshtml\"}")
_ = try TreeSitterLanguagePack.process(source: "<p>hi</p>\n", config: configObj)

```
