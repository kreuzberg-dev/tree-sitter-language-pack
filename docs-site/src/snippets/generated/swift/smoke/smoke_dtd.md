```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dtd\"}")
_ = try TreeSitterLanguagePack.process(source: "<!ELEMENT note (body)>", config: configObj)

```
