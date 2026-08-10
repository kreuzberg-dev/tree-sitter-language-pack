```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"solidity\"}")
_ = try TreeSitterLanguagePack.process(source: "pragma solidity ^0.8.0;\ncontract Main {}", config: configObj)

```
