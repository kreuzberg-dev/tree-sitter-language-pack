```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ocamllex\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
