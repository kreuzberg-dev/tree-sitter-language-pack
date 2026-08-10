```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hare\"}")
_ = try TreeSitterLanguagePack.process(source: "export fn main() void = void;", config: configObj)

```
