```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"styled\"}")
_ = try TreeSitterLanguagePack.process(source: "color: red;\n", config: configObj)

```
