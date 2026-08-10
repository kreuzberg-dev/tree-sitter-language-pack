```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"picat\"}")
_ = try TreeSitterLanguagePack.process(source: "main => true.\n", config: configObj)

```
