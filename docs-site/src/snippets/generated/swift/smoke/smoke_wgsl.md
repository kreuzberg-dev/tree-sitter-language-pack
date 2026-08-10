```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wgsl\"}")
_ = try TreeSitterLanguagePack.process(source: "@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", config: configObj)

```
