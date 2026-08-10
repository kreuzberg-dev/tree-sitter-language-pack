```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ninja\"}")
_ = try TreeSitterLanguagePack.process(source: "rule cc\n  command = cc $in -o $out", config: configObj)

```
