```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"scss\"}")
_ = try TreeSitterLanguagePack.process(source: "$color: red;\nbody { color: $color; }", config: configObj)

```
