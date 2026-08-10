```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rtf\"}")
_ = try TreeSitterLanguagePack.process(source: "{\\rtf1 hello}", config: configObj)

```
