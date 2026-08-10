```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"plantuml\"}")
_ = try TreeSitterLanguagePack.process(source: "@startuml\n@enduml\n", config: configObj)

```
