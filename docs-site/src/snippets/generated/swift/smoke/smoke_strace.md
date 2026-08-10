```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"strace\"}")
_ = try TreeSitterLanguagePack.process(source: "open(\"/x\", O_RDONLY) = 3\n", config: configObj)

```
