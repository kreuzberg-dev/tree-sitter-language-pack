```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jq\"}")
_ = try TreeSitterLanguagePack.process(source: ".[] | select(.key)", config: configObj)

```
