```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"batch\"}")
_ = try TreeSitterLanguagePack.process(source: "@echo off\necho hello", config: configObj)

```
