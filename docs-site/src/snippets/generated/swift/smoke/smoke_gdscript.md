```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gdscript\"}")
_ = try TreeSitterLanguagePack.process(source: "extends Node\nfunc _ready():\n\tpass", config: configObj)

```
