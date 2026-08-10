```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"swift\"}")
_ = try TreeSitterLanguagePack.process(source: "print(\"hello\")", config: configObj)

```
