```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cmake\"}")
_ = try TreeSitterLanguagePack.process(source: "cmake_minimum_required(VERSION 3.0)", config: configObj)

```
