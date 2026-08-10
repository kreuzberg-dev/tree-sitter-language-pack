```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"asm\"}")
_ = try TreeSitterLanguagePack.process(source: "mov eax, 1", config: configObj)

```
