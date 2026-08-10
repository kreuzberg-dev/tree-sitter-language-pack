```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pascal\"}")
_ = try TreeSitterLanguagePack.process(source: "program Hello; begin end.", config: configObj)

```
