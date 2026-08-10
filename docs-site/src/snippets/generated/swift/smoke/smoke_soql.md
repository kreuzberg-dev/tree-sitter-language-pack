```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"soql\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT Id FROM Account\n", config: configObj)

```
