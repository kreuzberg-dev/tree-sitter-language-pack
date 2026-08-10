```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"prisma\"}")
_ = try TreeSitterLanguagePack.process(source: "model User { id Int @id }", config: configObj)

```
