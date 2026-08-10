```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"mermaid\"}")
_ = try TreeSitterLanguagePack.process(source: "graph TD\nA --> B", config: configObj)

```
