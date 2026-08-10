```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cedar\"}")
_ = try TreeSitterLanguagePack.process(source: "permit(principal, action, resource);", config: configObj)

```
