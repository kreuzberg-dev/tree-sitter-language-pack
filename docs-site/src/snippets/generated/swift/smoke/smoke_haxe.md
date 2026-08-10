```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haxe\"}")
_ = try TreeSitterLanguagePack.process(source: "class Main { static function main() {} }", config: configObj)

```
