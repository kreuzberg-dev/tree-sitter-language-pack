```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gn\"}")
_ = try TreeSitterLanguagePack.process(source: "group(\"hello\") {}", config: configObj)

```
