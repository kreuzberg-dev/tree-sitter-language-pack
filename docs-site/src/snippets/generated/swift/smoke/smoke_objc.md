```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"objc\"}")
_ = try TreeSitterLanguagePack.process(source: "@interface Main @end", config: configObj)

```
