```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"zig\"}")
_ = try TreeSitterLanguagePack.process(source: "pub fn main() void {}", config: configObj)

```
