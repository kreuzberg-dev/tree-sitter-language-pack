```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"linkerscript\"}")
_ = try TreeSitterLanguagePack.process(source: "SECTIONS { .text : { *(.text) } }", config: configObj)

```
