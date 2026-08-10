```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"zsh\"}")
_ = try TreeSitterLanguagePack.process(source: "echo hello", config: configObj)

```
