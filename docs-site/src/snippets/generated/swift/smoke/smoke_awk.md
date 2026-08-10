```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"awk\"}")
_ = try TreeSitterLanguagePack.process(source: "BEGIN { print \"hello\" }", config: configObj)

```
