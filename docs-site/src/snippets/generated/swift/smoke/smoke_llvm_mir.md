```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"llvm_mir\"}")
_ = try TreeSitterLanguagePack.process(source: "---\nname: foo\n...\n", config: configObj)

```
