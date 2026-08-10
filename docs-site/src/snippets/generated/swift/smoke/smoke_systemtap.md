```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"systemtap\"}")
_ = try TreeSitterLanguagePack.process(source: "probe begin {}\n", config: configObj)

```
