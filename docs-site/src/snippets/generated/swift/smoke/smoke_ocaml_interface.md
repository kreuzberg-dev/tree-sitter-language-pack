```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ocaml_interface\"}")
_ = try TreeSitterLanguagePack.process(source: "val x : int", config: configObj)

```
